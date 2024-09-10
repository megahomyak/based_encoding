import json
import os
from abc import ABC, abstractmethod

class Entity(ABC):
    def __init__(self, parameters) -> None:
        super().__init__()
        self.parameters = parameters
    @abstractmethod
    def write_entity_contents(self, f):
        pass
    @abstractmethod
    def write_entity_type(self, f):
        pass
    @abstractmethod
    def write_base_method_contents(self, f):
        pass
    @abstractmethod
    def write_encode_method_contents(self, f):
        pass
    @abstractmethod
    def write_decode_method_contents(self, f):
        pass

class Number(Entity):
    def write_entity_type(self, f):
        f.write("struct")
    def write_entity_contents(self, f):
        f.write("pub value: BigUint")
    def write_base_method_contents(self, f):
        f.write("BigUint::from(")
        f.write(str(self.parameters["base"]))
        f.write("usize)")
    def write_encode_method_contents(self, f):
        f.write("write(number, &Self::base(), &self.value);")
    def write_decode_method_contents(self, f):
        f.write("Self { value: read(number, &Self::base()) }")

class String(Entity):
    def write_entity_type(self, f):
        f.write("struct")
    def write_entity_contents(self, f):
        f.write("pub contents: std::string::String")
    def write_base_method_contents(self, f):
        f.write("BigUint::from(256usize)")
    def write_encode_method_contents(self, f):
        f.write("write(number, &Self::base(), &BigUint::ZERO);")
        f.write("for byte in self.contents.as_bytes().iter().rev() {")
        f.write("write(number, &Self::base(), &BigUint::from(*byte));")
        f.write("}")
    def write_decode_method_contents(self, f):
        f.write("let mut bytes = Vec::new();")
        f.write("loop {")
        f.write("let byte = read(number, &Self::base());")
        f.write("if byte == BigUint::ZERO { break; }")
        f.write("bytes.push(u8::try_from(byte).unwrap());")
        f.write("}")
        f.write("Self { contents: std::string::String::from_utf8(bytes).unwrap() }")

class Struct(Entity):
    def write_entity_type(self, f):
        f.write("struct")
    def write_entity_contents(self, f):
        for field in self.parameters["fields"]:
            f.write("pub ")
            f.write(field["name"])
            f.write(":")
            f.write(field["type"])
            f.write(",")
    def write_base_method_contents(self, f):
        f.write(self.parameters["fields"][0]["type"])
        f.write("::base()")
    def write_encode_method_contents(self, f):
        for field in reversed(self.parameters["fields"]):
            f.write("self.")
            f.write(field["name"])
            f.write(".encode(number);")
    def write_decode_method_contents(self, f):
        for field in self.parameters["fields"]:
            f.write("let ")
            f.write(field["name"])
            f.write("=")
            f.write(field["type"])
            f.write("::decode(number);")
        f.write("Self {")
        for field in self.parameters["fields"]:
            f.write(field["name"])
            f.write(",")
        f.write("}")

class Enum(Entity):
    def write_entity_type(self, f):
        f.write("enum")
    def write_entity_contents(self, f):
        for variant in self.parameters["variants"]:
            f.write(variant["name"])
            f.write("(")
            if variant["associated_type"] != None:
                f.write(variant["associated_type"])
            f.write("),")
    def write_base_method_contents(self, f):
        f.write("BigUint::from(")
        f.write(str(len(self.parameters["variants"])))
        f.write("usize)")
    def write_encode_method_contents(self, f):
        f.write("match self {")
        for index, variant in enumerate(self.parameters["variants"]):
            f.write("Self::")
            f.write(variant["name"])
            f.write("(")
            if variant["associated_type"] != None:
                f.write("associated_value")
            f.write(") => {")
            if variant["associated_type"] != None:
                f.write("associated_value.encode(number);")
            f.write("write(number, &Self::base(), &BigUint::from(")
            f.write(str(index))
            f.write("usize));")
            f.write("}")
        f.write("}")
    def write_decode_method_contents(self, f):
        f.write("let num = read(number, &Self::base());")
        for index, variant in enumerate(self.parameters["variants"]):
            f.write("if num == BigUint::from(")
            f.write(str(index))
            f.write("usize) {")
            if variant["associated_type"] != None:
                f.write("let associated_value =")
                f.write(variant["associated_type"])
                f.write("::decode(number);")
            f.write("return Self::")
            f.write(variant["name"])
            f.write("(")
            if variant["associated_type"] != None:
                f.write("associated_value")
            f.write(");")
            f.write("}")
        f.write("unreachable!()")

class Sequence(Entity):
    def write_entity_type(self, f):
        f.write("struct")
    def write_entity_contents(self, f):
        f.write("pub items: Vec<")
        f.write(self.parameters["content_type"])
        f.write(">")
    def write_base_method_contents(self, f):
        f.write(self.parameters["content_type"])
        f.write("::base() + 1usize")
    def write_encode_method_contents(self, f):
        f.write("write(number, &Self::base(), &BigUint::ZERO);")
        f.write("for item in self.items.iter().rev() {")
        f.write("item.encode(number);")
        f.write("let item = read(number, &")
        f.write(self.parameters["content_type"])
        f.write("::base());")
        f.write("write(number, &Self::base(), &(item + 1usize));")
        f.write("}")
    def write_decode_method_contents(self, f):
        f.write("let mut items = Vec::new();")
        f.write("loop {")
        f.write("let num = read(number, &Self::base());")
        f.write("if num == BigUint::ZERO { break; }")
        f.write("write(number, &")
        f.write(self.parameters["content_type"])
        f.write("::base(), &(num - 1usize));")
        f.write("items.push(")
        f.write(self.parameters["content_type"])
        f.write("::decode(number)")
        f.write(");}")
        f.write("Self { items }")

def write_warning(input_file_name, f):
    f.write(f"// This file was generated automatically from {input_file_name}. DO NOT EDIT THIS FILE MANUALLY!\n")

def write_prelude(f):
    f.write("use crate::operations::{read, write};")
    f.write("use num_bigint::BigUint;")

def write_entity(entity, f):
    write_struct(entity, f)
    write_impl(entity, f)

def write_struct(entity, f):
    f.write("#[derive(Debug, PartialEq, Eq)]")
    f.write("pub ")
    entity.write_entity_type(f)
    f.write(" ")
    f.write(entity.parameters["name"])
    f.write("{")
    entity.write_entity_contents(f)
    f.write("}")

def write_impl(entity, f):
    f.write("impl ")
    f.write(entity.parameters["name"])
    f.write("{")
    write_impl_contents(entity, f)
    f.write("}")

def write_impl_contents(entity, f):
    write_base_method(entity, f)
    write_encode_method(entity, f)
    write_decode_method(entity, f)

def write_base_method(entity, f):
    f.write("#[allow(dead_code)]")
    f.write("pub fn base() -> BigUint {")
    entity.write_base_method_contents(f)
    f.write("}")

def write_encode_method(entity, f):
    f.write("pub fn encode(&self, number: &mut BigUint) {")
    entity.write_encode_method_contents(f)
    f.write("}")

def write_decode_method(entity, f):
    f.write("pub fn decode(number: &mut BigUint) -> Self {")
    entity.write_decode_method_contents(f)
    f.write("}")

def main():
    input_file_name = "entities.json"
    with open(input_file_name, "r", encoding="utf-8") as f:
        entities = json.load(f)

    with open("entities.rs", "w", encoding="utf-8") as f:
        write_warning(input_file_name, f)
        write_prelude(f)
        for parameters in entities:
            if parameters["type"] == "number":
                write_entity(Number(parameters), f)
            elif parameters["type"] == "string":
                write_entity(String(parameters), f)
            elif parameters["type"] == "struct":
                write_entity(Struct(parameters), f)
            elif parameters["type"] == "enum":
                write_entity(Enum(parameters), f)
            elif parameters["type"] == "sequence":
                write_entity(Sequence(parameters), f)
            else:
                raise ValueError(f"Unknown type `{parameters['type']}`")

    os.system("rustfmt entities.rs")

main()
