from .based_number import BasedDigit
from abc import ABC, abstractmethod
from typing import Self, Iterator
import itertools

class Entity(ABC):
    @abstractmethod
    def encode(self) -> Iterator[BasedDigit]:
        pass
    @abstractmethod
    def decode(self, based_number: Iterator[BasedDigit]) -> Self:
        pass

class Enum(Entity):
    def __init__(self, members: list[Entity]) -> None:
        self._members = members
    def encode(self) -> Iterator[BasedDigit]:
        return itertools.chain(
            *map(lambda member: member.encode(), self._members)
        )
    def decode(self, based_number: Iterator[BasedDigit]) -> Self:
        for num in based_number:

