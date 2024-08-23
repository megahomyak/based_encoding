from dataclasses import dataclass

@dataclass
class BasedDigit:
    value: int
    base: int

@dataclass
class BinaryString:
    bits: list[int]

class BasedNumber:
    def __init__(self, initial_digits: list[int], base: int):
        # I don't want these values to change independently
        self._reversed_digits = initial_digits[::-1]
        self._base = base

    @property
    def digits(self):
        return self._reversed_digits[::-1]

    @property
    def base(self):
        return self._base

    def convert(self, new_base: int) -> "BasedNumber":
        self_as_number = 0

        for index, digit in enumerate(self._reversed_digits):
            self_as_number += self.base ** index * digit

        digits = []
        while self_as_number:
            digits.append(int(self_as_number % new_base))
            self_as_number //= new_base

        return BasedNumber(initial_digits=digits[::-1])


class Base(int): pass

def encode(digits: list[BasedDigit]) -> BinaryString:
    base = 2
    output_digits = [1]
    for digit in digits:
        
    return output_digits
