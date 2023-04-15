from __future__ import annotations
from typing import Union, Set, Dict, List
from UnityPy.helpers.Tpk import TPKTYPETREE, TpkUnityNode

FORBIDDEN_NAMES = ("None", "True", "False", "pass", "from")
FORBIDDEN_CLASSES = ("Object", "float", "int", "bool", "string", "void", "object")
KNOWN_RENAMES = {
    "AnimatorState": "State",
    "AnimatorStateMachine": "StateMachine",
    "AnimatorStateTransition": "AnimatorTransition",
}

TPK_NODES = TPKTYPETREE.NodeBuffer.Nodes
TPK_STRINGS = TPKTYPETREE.StringBuffer.Strings

KNOWN_TYPE_IDS = {
    TPK_STRINGS.index(key): value
    for key, value in {
        "char": "int",
        "short": "int",
        "int": "int",
        "long long": "int",
        "unsigned short": "int",
        "unsigned int": "int",
        "unsigned long long": "int",
        "UInt8": "int",
        "UInt16": "int",
        "UInt32": "int",
        "UInt64": "int",
        "SInt8": "int",
        "SInt16": "int",
        "SInt32": "int",
        "SInt64": "int",
        "Type*": "int",
        "FileSize": "int",
        "float": "float",
        "double": "float",
        "bool": "bool",
        "string": "str",
        "TypelessData": "bytes",
    }.items()
}
MAP_ID = TPK_STRINGS.index("map")
ARRAY_ID = TPK_STRINGS.index("Array")
PAIR_ID = TPK_STRINGS.index("pair")


def main():
    gen = MainGenerator("Unity")
    for cls_info in TPKTYPETREE.ClassInformation.values():
        page = generate_class_pyi(cls_info, gen)
    if True:
        text = gen.to_str()
        with open("UnityPy/classes/classes.py", "wt", encoding="utf8") as f:
            f.write(text)
    print()


def get_string(string_id: int):
    string = TPK_STRINGS[string_id]
    if not string:
        return ""
    string = (
        string.replace(" ", "_")
        .replace("-", "_")
        .replace(".", "_")
        .rstrip("?")
        .replace("(int&)", "")
        .replace(":", "x")
    )
    if string in FORBIDDEN_NAMES or not string[0].isalpha():
        string = f"__{string}"
    return string


def get_node(node: Union[int, TpkUnityNode]) -> TpkUnityNode:
    return TPK_NODES[node] if isinstance(node, int) else node


def generate_class_pyi(cls_info, page_generator: MainGenerator = None):
    name = get_string(cls_info.Classes[0][1].Name)
    if name in FORBIDDEN_CLASSES:
        return None
    if not page_generator:
        page_generator = MainGenerator(name)
    main_generator = ClassGenerator(name, page_generator)
    main_generator.parents.add("Object")

    for version, cls in cls_info.Classes:
        if not cls:
            continue
        if cls.Base:
            parent = get_string(cls.Base)
            if parent:
                main_generator.parents.add(parent)
        if cls.ReleaseRootNode:
            main_generator.add_version(cls.ReleaseRootNode)

    return page_generator


class ClassGenerator:
    page: MainGenerator
    name: str
    fields: Dict[str, List[str]]
    nodes: List[int]
    parents: Set[int]
    # field-name: version: type
    # if different versions, merge via Union

    def __init__(self, name: str, page: MainGenerator) -> None:
        if name in page.classes:
            self.__dict__ = page.classes[name].__dict__
        else:
            self.page = page
            self.nodes = []
            self.name = name
            self.fields = {}
            self.parents = set()
            self.page.classes[name] = self

    def add_version(self, node: TpkUnityNode):
        node = get_node(node)
        # if node.Version in self.versions:
        #     return
        self.nodes.append(node)

        tuple_fields = {}
        for subnode_id in node.SubNodes:
            subnode = get_node(subnode_id)
            name = get_string(subnode.Name)

            if name.endswith("]"):
                name = name.rsplit("[", 1)[0]
                if name not in tuple_fields:
                    tuple_fields[name] = [1, subnode]
                else:
                    tuple_fields[name][0] += 1
                continue

            if name in self.fields:
                self.fields[name].append(self.get_type(subnode))
            else:
                self.fields[name] = [self.get_type(subnode)]

        for key, (count, subnode) in tuple_fields.items():
            type = self.get_type(subnode)
            type = f"Tuple[{', '.join([type]*count)}]"
            if key in self.fields:
                self.fields[key].append(type)
            else:
                self.fields[key] = [type]

    def get_type(self, node: Union[int, TpkUnityNode]):
        if isinstance(node, int):
            node = get_node(node)

        typ = KNOWN_TYPE_IDS.get(node.TypeName, None)
        if typ:
            return typ
        if node.TypeName == MAP_ID:
            subnode = get_node(node.SubNodes[0])
            first, second = get_node(subnode.SubNodes[1]).SubNodes
            return f"List[Tuple[{self.get_type(first)}, {self.get_type(second)}]]"
        if node.TypeName == PAIR_ID:
            node_1 = get_node(node.SubNodes[0])
            node_2 = get_node(node.SubNodes[1])
            return f"Tuple[{self.get_type(node_1)}, {self.get_type(node_2)}]"
        elif (
            len(node.SubNodes) == 1 and get_node(node.SubNodes[0]).TypeName == ARRAY_ID
        ):
            subnode = get_node(node.SubNodes[0])
            typenode = get_node(subnode.SubNodes[1])
            return f"List[{self.get_type(typenode)}]"
        # class
        # TODO - generate new class pyi
        name = get_string(node.TypeName)
        if name.startswith("PPtr<"):
            subtype = name[5:-1]
            self.page.unknown_classes.add(subtype)
            # TODO - add Generic[T] to PPtr as super class
            # https://stackoverflow.com/questions/60202915/set-typehint-for-custom-subclass-of-list-typeerror-type-object-is-not-subsc
            return f"PPtr[{name[5:-1]}]"
        # check if main classes -> import
        # else generate within this
        cls = self.page.classes.get(name, ClassGenerator(name, self.page))
        cls.add_version(node)
        return name

    def field_to_str(self, field_value: str):
        ftypes = self.fields[field_value]
        stypes = set(ftypes)

        if len(stypes) == 1:
            typ = stypes.pop()
        else:
            typ = f"Union[{', '.join(stypes)}]"

        if len(ftypes) < len(self.nodes):
            return f"Optional[{typ}]"
        return typ

    def to_str(self) -> str:
        parents = None
        if self.parents:
            if len(self.parents) == 1:
                parents = next(iter(self.parents))
            else:
                # resolve parents, pick the "lowest" one,
                # so if parents are A and B, but A is also parent of B, ignore A and pick B
                parents = self.parents
                stack_check = list(parents)
                while stack_check:
                    item = stack_check.pop()
                    cls_i = self.page.classes.get(item, None)
                    if cls_i is None:
                        continue
                    for parent in cls_i.parents:
                        if parent in parents:
                            parents.remove(parent)
                        stack_check.append(parent)

                parents = f"{', '.join(parents)}"

        cls_def = f"class {self.name}({parents}):" if parents else f"class {self.name}:"

        if len(self.fields) == 0:
            return f"{cls_def}\n    pass"

        return "\n".join(
            [
                "@define",
                cls_def,
                *[
                    f"    {field}: {self.field_to_str(field)} = None"
                    for field in self.fields
                ],
            ]
        )


class MainGenerator:
    classes: Dict[str, ClassGenerator]  # all classes within the page
    unknown_classes: Set[
        str
    ]  # all classes that are unknown if they have to be imported or not
    inheritance_table: Dict[str, str]  # class name -> super class name

    def __init__(self, name) -> None:
        self.name = name
        self.classes = {}
        self.unknown_classes = set()
        self.imports = set()
        self.inheritance_table = {}

    def add_import(self, import_str: str):
        self.imports.add(import_str)

    def to_str(self):
        classes = sorted(self.classes.values(), key=lambda x: x.name)
        # added = set()
        # classes_ordered = []
        # added = set(["Object"])
        # # oreder classes based on inheritance
        # while classes:
        #     print(len(classes))
        #     cls = classes.pop(0)
        #     if all(p in added for p in cls.parents):
        #         classes_ordered.append(cls)
        #         added.add(cls.name)
        #     else:
        #         classes.append(cls)

        # 2. save
        return "\n".join(
            [
                "struct PPtr<T>{};",
                "\n",
                "\n\n".join(cls.to_str() for cls in classes),
                # 2.3 renames
                "\n",
                "\n".join(f"use {v} as {k};" for k, v in KNOWN_RENAMES.items()),
            ]
        )


if __name__ == "__main__":
    main()
