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

    def read(self, base: ValidBase) -> UnsignedInt:
        self.convert(base)
        try:
            return self._digits.pop()
        except IndexError:
            return UnsignedInt(0)

def based_digit(valid_base: int, value: int):
    return BasedDigit(ValidBase(UnsignedInt(valid_base)), UnsignedInt(value))

def valid_base(value: int):
    return ValidBase(UnsignedInt(value))

def main():
    based_number = BasedNumber()
    based_number.write(based_digit(valid_base=20, value=0))
    based_number.write(based_digit(3, 2))
    based_number.write(based_digit(2, 1))
    print(based_number.read(valid_base(2)))
    print(based_number.read(valid_base(3)))
    print(based_number.read(valid_base(20)))

main()
