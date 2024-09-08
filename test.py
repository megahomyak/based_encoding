from . import complex

def based_digit(valid_base: int, value: int):
    return BasedDigit(ValidBase(UnsignedInt(valid_base)), UnsignedInt(value))

def valid_base(value: int):
    return ValidBase(UnsignedInt(value))
based_number = BasedNumber()
based_number.write(based_digit(valid_base=20, value=0))
based_number.write(based_digit(3, 2))
based_number.write(based_digit(2, 1))
print(based_number.read(valid_base(2)))
print(based_number.read(valid_base(3)))
print(based_number.read(valid_base(20)))
