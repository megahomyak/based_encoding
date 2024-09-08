from .based_number import BasedDigit, UnsignedInt, ValidBase

def based_digit(valid_base: int, value: int):
    return BasedDigit(ValidBase(UnsignedInt(valid_base)), UnsignedInt(value))

def valid_base(value: int):
    return ValidBase(UnsignedInt(value))
