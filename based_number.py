from dataclasses import dataclass

class BaseTooSmall(Exception):
    pass

class ValueBiggerThanBase(Exception):
    pass

class IntegerIsSigned(Exception):
    pass

class UnsignedInt(Exception):
    def __init__(self, value: int) -> None:
        if value < 0:
            raise IntegerIsSigned()
        self._value = value

    @property
    def value(self) -> int:
        return self._value

class ValidBase:
    def __init__(self, value: UnsignedInt) -> None:
        if value.value < 1:
            raise BaseTooSmall()
        self._value = value

    @property
    def value(self) -> UnsignedInt:
        return self._value

    def increase(self, n: UnsignedInt) -> "ValidBase":
        return ValidBase(UnsignedInt(self.value.value + n.value))

class BasedDigit:
    def __init__(self, base: ValidBase, value: UnsignedInt) -> None:
        if base.value.value <= value.value:
            raise ValueBiggerThanBase()
        self._base = base
        self._value = value

    @property
    def base(self) -> ValidBase:
        return self._base

    @property
    def value(self) -> UnsignedInt:
        return self._value

    def increase(self, n: UnsignedInt) -> "BasedDigit":
        return BasedDigit(
            self.base.increase(n),
            UnsignedInt(self.value.value + n.value),
        )

@dataclass
class BasedNumber:
    def __init__(self) -> None:
        self._digits: list[UnsignedInt] = []
        self._base: ValidBase = ValidBase(UnsignedInt(1))

    def digits(self):
        return iter(self._digits)

    @property
    def base(self):
        return self._base

    def convert(self, new_base: ValidBase):
        if new_base == self.base:
            return
        self_as_number = 0

        for index, digit in enumerate(self._digits[::-1]):
            self_as_number += self._base.value.value ** index * digit.value

        reversed_digits = []
        while self_as_number:
            reversed_digits.append(UnsignedInt(self_as_number % new_base.value.value))
            self_as_number //= new_base.value.value

        self._base = new_base
        self._digits = reversed_digits[::-1]

    def write(self, based_digit: BasedDigit):
        if len(self._digits) == 0 and based_digit.value.value == 0:
            return
        self.convert(based_digit.base)
        self._digits.append(based_digit.value)

    def read_digit(self, base: ValidBase) -> UnsignedInt:
        self.convert(base)
        try:
            return self._digits.pop()
        except IndexError:
            return UnsignedInt(0)
