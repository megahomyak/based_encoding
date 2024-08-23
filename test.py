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
            reversed_digits.append(int(self_as_number % new_base))
            self_as_number //= new_base

        self.base = new_base
        self.digits = reversed_digits[::-1]

@dataclass
class BasedDigit:
    value: int
    base: int

Bit = int
Base = int

def encode(values: list[int], bases: list[Base]) -> list[Bit]:
    final_number = BasedNumber(digits=[1], base=2)
    for value, base in zip(values, bases):
        final_number.convert(new_base=base)
        final_number.digits.append(value)
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
    bases = [Base(20), Base(3), Base(2)]
    print(f"{bases = }")
    values = [18, 2, 1]
    print(f"{values = }")
    encoded_values = encode(values, bases)
    print(f"{encoded_values = }")
    decoded_values = decode(encoded_values, bases)
    print(f"{decoded_values = }")

main()
