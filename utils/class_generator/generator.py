from __future__ import annotations
import os
import re
from typing import List, Dict, TextIO, Set, Tuple
from dataclasses import dataclass
from collections import defaultdict
import json

from config import METADATA_PATH, TYPETREEDUMPS_PATH
from field_type import Typ, EnumTyp, IntTyp, ArrayTyp, FloatTyp, PairTyp, PPtrTyp
from metadata_scraper import UnityClass

OFFICIAL_DOC_URL = "https://docs.unity3d.com/ScriptReference/{}.html"
# 0 - indent
# 1 - unsigned?
# 2 - incomplete type
# 3 - type
BASE_TYPE_MAP = {
    "char": "char",
    "short": "i16",
    "int": "i32",
    "long long": "i64",
    "unsigned short": "u16",
    "unsigned int": "u32",
    "unsigned long long": "u64",
    "UInt8": "u8",
    "UInt16": "u16",
    "UInt32": "u32",
    "UInt64": "u64",
    "SInt8": "i8",
    "SInt16": "i16",
    "SInt32": "i32",
    "SInt64": "i64",
    "Type*": "i32",
    "FileSize": "i32",
    "float": "f32",
    "double": "f64",
    "bool": "bool",
    "string": "String",
    "TypelessData": "Vec<u8>",
    # useless data types
    "NamedObject": "String",  # NamedObject only contains m_Name: String
}
FLOAT_TYPES = ["f32", "f64"]
INT_TYPES = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64"]
FORBIDDEN_TYPE_NAMES = ["type", "loop"]


def main():
    version_dir = os.path.join(TYPETREEDUMPS_PATH, "InfoJson")
    clz_handler = AllClassHandler()
    clz_handler.load_metadata(METADATA_PATH)

    # get all versions
    versions = os.listdir(version_dir)
    versions.sort(key=version_string_to_int)

    for file in versions:
        print(f"Loading {file}")
        with open(os.path.join(version_dir, file), "rt", encoding="utf8") as f:
            info = json.load(f)

        for clz in info["Classes"]:
            if clz["ReleaseRootNode"] is not None:
                clz_handler.process_class_nodes(
                    NodeH.from_dict(clz["ReleaseRootNode"]), file[:-5]
                )

    clz_handler.apply_hotfixes()

    with open("classes.rs", "wt", encoding="utf8") as file:
        clz_handler.write_rs(file)


class AllClassHandler:
    classes: Dict[str, Class]
    metadata: Dict[str, Dict[str, str]]
    known_enums: Set[str]

    def __init__(self):
        self.classes = {}
        self.metadata = {}
        self.known_enums = set()

    def process_class_nodes(self, node: NodeH, version: str):
        clz = self.classes.get(node.TypeName)

        if clz is None:
            Class(node, version, self.classes)
        else:
            clz.add_version(node, version, self.classes)

    def merged_identical_classes(self):
        pass

    def load_metadata(self, mt_fp: str):
        with open(mt_fp, "rt", encoding="utf8") as f:
            metainfo = json.load(f)
        self.metadata = {info["title"]: UnityClass.from_dict(info) for info in metainfo}

    def write_rs(self, f: TextIO):
        f.writelines(
            [
                "#![allow(warnings)]\n",
                "use crate::objects::PPtr;\n",
                "use serde::{Deserialize, Serialize};\n",
                "\n",
            ]
        )

        for cls in sorted(self.classes.values(), key=lambda x: x.name):
            meta = self.metadata.get(cls.name)
            f.write("\n".join(cls.generate_rust_doc(meta)))
            f.write("\n")
            str_struct = cls.generate_rust_struct(meta)
            f.write(self.patch_enums(str_struct))
            f.write("\n\n")

    def patch_enums(self, text: str):
        enums = re.findall(r"(Enum<\|(.+?)\|>)", text)
        enum_to_create = []

        def format_name(name: str):
            return re.sub("[<>\(\), ]", "_", name.strip("<>()"))

        for full_enum, enum_typ in enums:
            enum_name = f"Enum_{format_name(enum_typ)}"
            if enum_name not in self.known_enums:
                self.known_enums.add(enum_name)
                parts = enum_typ.split(", ")
                i = 0
                while i < len(parts):
                    if parts[i].startswith("("):
                        j = i + 1
                        while not parts[j].endswith(")"):
                            j += 1
                        parts[i] = ", ".join(parts[i : j + 1])
                        del parts[i + 1 : j + 1]
                    i += 1

                enum_to_create.append((enum_name, parts))
            text = text.replace(full_enum, enum_name)

        enums = []
        for enum_name, parts in enum_to_create:

            def format_type(t: str):
                if t.startswith("Vec"):
                    return f"Vec({t})"
                else:
                    return f"{format_name(t)}({t})"

            enum_text = "\n".join(
                [
                    "#[derive(Debug, Serialize, Deserialize)]",
                    "#[serde(untagged)]",
                    f"pub enum {enum_name} {{",
                    f"  {', '.join(map(format_type, parts))}",
                    "}\n",
                ]
            )
            enums.append(enum_text)
        else:
            text = "\n\n".join([text] + enums)

        return text

    def apply_hotfixes(self):
        # Clip & OffsetPtr refer to each other
        # -> infinite size
        # Hotfix: Clip.m_Binding: OffsetPtr -> Option<Box<OffsetPtr>>
        clip_m_binding = self.classes["Clip"].fields["m_Binding"]
        for typ, versions in clip_m_binding.types.items():
            if isinstance(typ, Typ) and typ.value == "OffsetPtr":
                typ.value = "Box<OffsetPtr>"
                break


@dataclass
class Class:
    """
    A class is a collection of nodes that have the same name.

    Properties:
        name: The name of the class
        fields: A dictionary of fields, where the key is the name of the field
            and the value is a Field object.
        is_main_class: Whether this class is used as first level class.
        is_sub_class: Whether this class is used as a sub class.
        min_version: The minimum version this class was found in.
    """

    name: str
    fields: Dict[str, Field]
    is_main_class: bool
    is_sub_class: bool
    min_version: str
    versions: List[str]

    def __init__(self, nodeh: NodeH, version: str, classes: Dict[str, Class]):
        self.name = nodeh.TypeName
        self.min_version = version
        self.versions = [version]

        base_level = nodeh.Level
        if base_level == 0:
            self.is_main_class = True
            self.is_sub_class = False
        else:
            self.is_main_class = False
            self.is_sub_class = True

        self.fields = {
            generate_rust_name(child_nodeh.Name): Field(child_nodeh, version, classes)
            for child_nodeh in nodeh.SubNodes
        }
        classes[self.name] = self

    def add_version(self, nodeh: NodeH, version: str, classes: Dict[str, Class]):
        if version in self.versions:
            return
        self.versions.append(version)
        base_level = nodeh.Level
        if base_level == 0:
            self.is_main_class = True
        else:
            self.is_sub_class = True

        known_fields = set(self.fields.keys())
        for child_nodeh in nodeh.SubNodes:
            field = self.fields.get(generate_rust_name(child_nodeh.Name))
            if field is None:
                self.fields[child_nodeh.Name] = Field(
                    child_nodeh, version, classes, True
                )
            else:
                field.add_version(child_nodeh, version, classes)

            known_fields.discard(child_nodeh.Name)

        # make deprecated fields optional
        for field_name in known_fields:
            self.fields[field_name].optional = True

    def generate_rust_struct(self, metadata: UnityClass = None) -> str:
        # 1. prefilter fields - check if we have a field with the same cleaned up name
        fields = sorted(
            self.fields.items(),
            key=lambda x: (x[1].optional, x[0]),
        )

        if metadata and metadata.properties:
            metadoc = {key.lower(): val for key, val in metadata.properties.items()}
        else:
            metadoc = {}

        field_lines = []
        for fname, field in fields:
            if metadoc:
                names = set(
                    map(
                        str.lower,
                        [
                            fname,
                            fname.lstrip("m_"),
                            fname.replace(" ", ""),
                            fname.replace("_", ""),
                            fname.replace(" ", "_"),
                        ],
                    )
                )
                doc = next(
                    (metadoc.get(name, None) for name in names if name in metadoc), None
                )
                if doc is not None:
                    doc = doc.strip().replace("\n", "")
                    field_lines.append(f"  /**{doc}*/")

            field_doc = field.generate_rust_doc()
            if field.optional or "PPtr" in field_doc:
                field_lines.append(field_doc)

            field_lines.append(field.generate_rust_field())

        return "\n".join(
            [
                "#[derive(Debug, Serialize, Deserialize)]",
                f"pub struct {self.name} {{",
                "\n".join(field_lines),
                "}",
            ]
        )

    def generate_rust_doc(self, metadata: UnityClass = None) -> List[str]:
        lines = [
            f"/// {self.name} is a {'sub' if not self.is_main_class else ''} class of the Unity engine since version {self.min_version}.",
        ]
        if metadata is not None:
            if metadata is None:
                lines.append(
                    f"/// [Unity's scripting documentation]({OFFICIAL_DOC_URL.format(metadata.link)})"
                )
            else:
                lines.append(
                    f"/// Exert from [Unity's scripting documentation]({OFFICIAL_DOC_URL.format(metadata.link)}):"
                )
                lines.append(f"/**\n{metadata.description}\n*/")
        return lines


@dataclass
class Field:
    name: str
    aliases: List[str]
    types: Dict[str, List[str]]
    optional: bool
    min_version: str

    def __init__(
        self,
        nodeh: NodeH,
        version: str,
        classes: Dict[str, Class],
        is_optional: bool = False,
    ):
        self.name = generate_rust_name(nodeh.Name)
        self.aliases = []
        if nodeh.Name != self.name:
            self.aliases.append(nodeh.Name)

        self.types = defaultdict(list)
        self.optional = is_optional
        self.min_version = version

        self.types[nodeh.get_rust_type(classes, version)].append(version)

    def add_version(self, nodeh: NodeH, version: str, classes: Dict[str, Class]):
        if nodeh.Name != self.name and nodeh.Name not in self.aliases:
            self.aliases.append(nodeh.Name)
        self.types[nodeh.get_rust_type(classes, version)].append(version)

    @staticmethod
    def merge(field1: Field, field2: Field, name: str) -> Field:
        min_version = min(
            field1.min_version,
            field2.min_version,
            key=version_string_to_int,
        )
        # spawn new field
        new_field = Field(
            NodeH.from_dict({"Name": name, "TypeName": "i32"}),
            min_version,
            {},
            field1.optional or field2.optional,
        )
        # merge types
        new_field.types = field1.types.copy()
        for typ, versions in field2.types.items():
            if typ in new_field.types:
                new_field.types[typ].extend(versions)
                new_field.types[typ].sort(key=version_string_to_int)
            else:
                new_field.types[typ] = versions
        return new_field

    def generate_rust_type(self) -> str:
        types = list(self.types.keys())
        if len(types) == 1:
            typ = types[0]
        else:
            if len(set(type(t) for t in types)) == 1:
                typ = types[0].find_common_superior(*types)
            else:
                typ = EnumTyp(types)

        typ = typ.typ_str()
        if self.optional:
            typ = f"Option<{typ}>"

        return typ

    def generate_rust_doc(self) -> str:
        return "  /// " + "; ".join(
            [
                f"{typ.doc_str()}: ({versions[0]} - {versions[-1]})"
                for typ, versions in self.types.items()
            ]
        )

    def generate_rust_field(self) -> str:
        typ = self.generate_rust_type()
        if self.aliases:
            return f"""  #[serde({', '.join(f'alias = "{alias}"' for alias in self.aliases)})]\n  pub {self.name}: {typ},"""
        return f"  pub {self.name}: {typ},"


@dataclass
class NodeH:
    TypeName: str
    Name: str
    Level: int
    ByteSize: int
    Index: int
    Version: int
    TypeFlags: int
    MetaFlag: int
    SubNodes: List[NodeH]

    @classmethod
    def from_dict(cls, data):
        return cls(
            TypeName=data.get("TypeName"),
            Name=data.get("Name"),
            Level=data.get("Level"),
            ByteSize=data.get("ByteSize"),
            Index=data.get("Index"),
            Version=data.get("Version"),
            TypeFlags=data.get("TypeFlags"),
            MetaFlag=data.get("MetaFlag"),
            SubNodes=[cls.from_dict(node) for node in data.get("SubNodes", [])],
        )

    def get_rust_type(self, classes: Dict[str, Class], version: str) -> Typ:
        """
        Returns the rust type of this node.

        Returns:
            Returns the rust type of this node.
            If the type is a custom class, None is returned.
        """
        typ = BASE_TYPE_MAP.get(self.TypeName)
        if typ is not None:
            if typ in INT_TYPES:
                return IntTyp(typ)
            elif typ in FLOAT_TYPES:
                return FloatTyp(typ)
            else:
                return Typ(typ)

        # map & vector
        if len(self.SubNodes) == 1 and self.SubNodes[0].TypeName == "Array":
            # Children:
            #   Array Array
            #       SInt32 size
            #       Typ data
            return ArrayTyp(
                self.SubNodes[0].SubNodes[1].get_rust_type(classes, version)
            )
        elif self.TypeName == "pair":
            # Children:
            #   Typ1 first
            #   Typ2 second
            return PairTyp(
                self.SubNodes[0].get_rust_type(classes, version),
                self.SubNodes[1].get_rust_type(classes, version),
            )
        elif self.TypeName.startswith("PPtr<"):
            return PPtrTyp(self.TypeName)
        else:
            # custom class
            typ = self.TypeName
            clz = classes.get(typ)
            if clz is None:
                Class(self, version, classes)
            else:
                clz.add_version(self, version, classes)
            return Typ(typ)


def version_string_to_int(version: str) -> Tuple[int]:
    return tuple(map(int, re.findall(r"\d+", version)))


def generate_rust_name(name) -> str:
    name = re.sub(r"[: \[\]\?\.-]", "_", name)
    if name.startswith("(int&)"):
        name = name[6:]
    if name[0].isdigit() or name in FORBIDDEN_TYPE_NAMES:
        name = f"_{name}"

    return name


if __name__ == "__main__":
    main()
