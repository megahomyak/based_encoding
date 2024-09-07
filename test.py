from dataclasses import dataclass

@dataclass
class BasedNumber:
    digits: list[int]
    base: int

    def convert(self, new_base):
        self_as_number = 0

        for index, digit in enumerate(self.digits[::-1]):
            self_as_number += self.base ** index * digit

        reversed_digits = []
        while self_as_number:
            reversed_digits.append(self_as_number % new_base)
            self_as_number //= new_base

        self.base = new_base
        self.digits = reversed_digits[::-1]

@dataclass
class BasedDigit:
    value: int
    base: int

def get_bases(based_digits: list[BasedDigit]):
    bases = []
    for based_digit in based_digits:
        bases.append(based_digit.base)
    return bases

Bit = int
Base = int

def encode(based_digits: list[BasedDigit]) -> list[Bit]:
    final_number = BasedNumber(digits=[1], base=2)
    for based_digit in based_digits:
        final_number.convert(new_base=based_digit.base)
        final_number.digits.append(based_digit.value)
    final_number.convert(new_base=2)
    final_number.digits.pop(0) # Leftmost "1"
    return final_number.digits

def decode(encoded_values: list[Bit], bases: list[Base]) -> list[int]:
    final_number = BasedNumber(digits=[1] + encoded_values, base=2)
    values = []
    for base in bases[::-1]:
        final_number.convert(base)
        values.append(final_number.digits.pop())
    return values[::-1]

def main():
    based_digits = [
        BasedDigit(base=20, value=18),
        BasedDigit(base=3, value=2),
        BasedDigit(base=2, value=1),
    ]
    bases = get_bases(based_digits)
    print(f"{based_digits = }")
    encoded_values = encode(based_digits)
    print(f"{encoded_values = }")
    decoded_values = decode(encoded_values, bases)
    print(f"{decoded_values = }")

main()
