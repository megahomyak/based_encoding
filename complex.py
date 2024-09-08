from .based_number import BasedDigit, ValidBase, UnsignedInt
from abc import ABC, abstractmethod
from typing import Iterator, Any
from itertools import chain
from .utils import valid_base, based_digit

class NumberInput(ABC):
    @abstractmethod
    def read_digit(self, base: ValidBase) -> UnsignedInt:
        pass

class Entity(ABC):
    @abstractmethod
    def encode(self, value: Any) -> Iterator[BasedDigit]:
        pass
    @abstractmethod
    def decode(self, number_input: NumberInput) -> Any:
        pass

class Struct(Entity):
    def __init__(self, members: list[Entity]) -> None:
        self._members = members
    def encode(self, value: list) -> Iterator[BasedDigit]:
        return chain(
            *(
                member.encode(value_item)
                for value_item, member in zip(value, self._members)
            )
        )
    def decode(self, number_input: NumberInput) -> Any:
        results = []
        for member in reversed(self._members):
            results.append(member.decode(number_input))
        return results

class Enum(Entity):
    def __init__(self, members: set[str]) -> None:
        self._members_as_list = list(members)
        del members
        self._member_to_number = {
            item: UnsignedInt(i)
            for i, item in enumerate(self._members_as_list)
        }
        self._base = valid_base(len(self._members_as_list))
    def encode(self, value: str) -> Iterator[BasedDigit]:
        return iter([BasedDigit(self._base, self._member_to_number[value])])
    def decode(self, number_input: NumberInput) -> Any:
        return self._members_as_list[number_input.read_digit(self._base).value]

class Sequence(Entity):
    def __init__(self, member: Entity) -> None:
        self._member = member
    def encode(self, value: Any) -> Iterator[BasedDigit]:
        # Think about this
        pass
    def decode(self, number_input: NumberInput) -> Any:
        class IncreasedNumberInput:
            pass
        self._member.decode()
