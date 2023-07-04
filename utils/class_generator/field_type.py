from __future__ import annotations
from typing import List


class Typ:
    value: str

    def __init__(self, value: str) -> None:
        self.value = value

    @staticmethod
    def find_common_superior(*types: Typ) -> Typ:
        unique_types = set(types)
        if len(unique_types) == 1:
            return types[0]
        else:
            return EnumTyp(types)

    def __eq__(self, other) -> bool:
        if isinstance(other, Typ):
            return self.value == other.value
        else:
            return False

    def __hash__(self) -> int:
        return hash(self.value)

    def __repr__(self) -> str:
        self.doc_str

    def typ_str(self) -> str:
        return self.value

    def doc_str(self) -> str:
        return self.typ_str()


class PPtrTyp(Typ):
    value: str

    def __init__(self, value: str) -> None:
        if value.startswith("PPtr<"):
            self.value = value[5:-1]
        else:
            self.value = value

    def typ_str(self) -> str:
        return "PPtr"

    def doc_str(self) -> str:
        return f"PPtr<{self.value}>"

    @staticmethod
    def find_common_superior(*types: PPtrTyp) -> Typ:
        unique_types = set(types)
        if len(unique_types) == 1:
            return types[0]
        else:
            return PPtrTyp(" | ".join(t.value for t in types))


class EnumTyp(Typ):
    types: List[Typ]

    def __init__(self, types: List[Typ]) -> None:
        self.types = types

    def __eq__(self, other) -> bool:
        if isinstance(other, EnumTyp):
            return len(self.types) == len(other.types) and all(
                s == o for s, o in zip(self.types, other.types)
            )
        else:
            return False

    def typ_str(self) -> str:
        return f"Enum<|{', '.join(t.typ_str() for t in self.types)}|>"

    def __hash__(self) -> int:
        return hash(self.doc_str())


class IntTyp(Typ):
    signed: bool
    size: int

    def __init__(self, value: str) -> None:
        self.signed = value.startswith("i")
        self.size = int(value[1:])

    @staticmethod
    def find_common_superior(*types: IntTyp) -> IntTyp:
        max_signed = max(
            (t for t in types if t.signed), key=lambda x: x.size, default=None
        )
        max_unsigned = max(
            (t for t in types if not t.signed), key=lambda x: x.size, default=None
        )
        if max_signed is None:
            return max_unsigned
        elif max_unsigned is None:
            return max_signed
        else:
            return IntTyp(f"i{max(max_signed.size, max_unsigned.size * 2)}")

    def __eq__(self, other) -> bool:
        if isinstance(other, IntTyp):
            return self.signed == other.signed and self.size == other.size
        else:
            return False

    def typ_str(self) -> str:
        return f"{'i' if self.signed else 'u'}{self.size}"

    def __hash__(self) -> int:
        return hash(self.doc_str())


class FloatTyp(Typ):
    size: int

    def __init__(self, value: str) -> None:
        self.size = int(value[1:])

    @staticmethod
    def find_common_superior(*types: FloatTyp) -> FloatTyp:
        return max(types, key=lambda x: x.size, default=None)

    def __eq__(self, other) -> bool:
        if isinstance(other, FloatTyp):
            return self.size == other.size
        else:
            return False

    def typ_str(self) -> str:
        return f"f{self.size}"

    def __hash__(self) -> int:
        return hash(self.doc_str())


class ArrayTyp(Typ):
    typ: Typ

    def __init__(self, typ: Typ) -> None:
        self.typ = typ

    @staticmethod
    def find_common_superior(*types: ArrayTyp) -> ArrayTyp:
        # 1. check if all subtypes are the same
        subtypes = set(t.typ for t in types)
        if len(subtypes) == 1:
            return ArrayTyp(subtypes.pop())
        elif len(set(map(type, subtypes))) == 1:
            return ArrayTyp(types[0].typ.find_common_superior(*subtypes))
        else:
            return ArrayTyp(EnumTyp(subtypes))

    def __eq__(self, other) -> bool:
        if isinstance(other, ArrayTyp):
            return self.typ == other.typ
        else:
            return False

    def typ_str(self) -> str:
        return f"Vec<{self.typ.typ_str()}>"

    def doc_str(self) -> str:
        return f"Vec<{self.typ.doc_str()}>"

    def __hash__(self) -> int:
        return hash(self.doc_str())


class PairTyp(Typ):
    first: Typ
    second: Typ

    def __init__(self, first: Typ, second: Typ) -> None:
        self.first = first
        self.second = second

    @staticmethod
    def find_common_superior(*types: List[PairTyp]) -> PairTyp:
        # 1. check if all first types are the same
        first_types = set(t.first for t in types)
        if len(first_types) == 1:
            first = first_types.pop()
        elif len(set(map(type, first_types))) == 1:
            first = types[0].first.find_common_superior(*first_types)
        else:
            first = EnumTyp(first_types)
        # 2. check if all second types are the same
        second_types = set(t.second for t in types)
        if len(second_types) == 1:
            second = second_types.pop()
        elif len(set(map(type, second_types))) == 1:
            second = types[0].second.find_common_superior(*second_types)
        else:
            second = EnumTyp(second_types)
        return PairTyp(first, second)

    def __eq__(self, other) -> bool:
        if isinstance(other, PairTyp):
            return self.first == other.first and self.second == other.second
        else:
            return False

    def typ_str(self) -> str:
        return f"({self.first.typ_str()}, {self.second.typ_str()})"

    def doc_str(self) -> str:
        return f"({self.first.doc_str()}, {self.second.doc_str()})"

    def __hash__(self) -> int:
        return hash(self.doc_str())
