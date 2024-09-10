import jinja2
import json
import os

TEMPLATE = jinja2.Template(
"""
// This file was generated automatically from {{ input_file_name }}. DO NOT EDIT THIS FILE MANUALLY!
use crate::operations::{read, write};
use num_bigint::BigUint;

{% for entity in entities %}
    #[derive(Debug, PartialEq, Eq)]
    {%- if entity.type == "number" %}
        pub struct {{ entity.name }} {
            pub value: BigUint,
        }

        impl {{ entity.name }} {
            #[allow(dead_code)]
            pub fn base() -> BigUint {
                BigUint::from({{ entity.base }}usize)
            }
            pub fn encode(&self, number: &mut BigUint) {
                write(number, &Self::base(), &self.value);
            }
            pub fn decode(number: &mut BigUint) -> Self {
                Self { value: read(number, &Self::base()) }
            }
        }
    {% elif entity.type == "string" %}
        pub struct {{ entity.name }} {
            pub contents: std::string::String,
        }

        impl {{ entity.name }} {
            #[allow(dead_code)]
            pub fn base() -> BigUint {
                BigUint::from(256usize)
            }
            pub fn encode(&self, number: &mut BigUint) {
                write(number, &Self::base(), &BigUint::ZERO);
                for byte in self.contents.as_bytes().iter().rev() {
                    write(number, &Self::base(), &BigUint::from(*byte));
                }
            }
            pub fn decode(number: &mut BigUint) -> Self {
                let mut bytes = Vec::new();
                loop {
                    let byte = read(number, &Self::base());
                    if byte == BigUint::ZERO {
                        break;
                    }
                    bytes.push(u8::try_from(byte).unwrap());
                }
                Self { contents: std::string::String::from_utf8(bytes).unwrap() }
            }
        }
    {% elif entity.type == "struct" %}
        pub struct {{ entity.name }} {
            {% for field in entity.fields -%}
                pub {{ field.name }}: {{ field.type }},
            {% endfor %}
        }

        impl {{ entity.name }} {
            #[allow(dead_code)]
            pub fn base() -> BigUint {
                {{ entity.fields[0].type }}::base()
            }
            pub fn encode(&self, number: &mut BigUint) {
                {% for field in entity.fields|reverse -%}
                    self.{{ field.name }}.encode(number);
                {% endfor %}
            }
            pub fn decode(number: &mut BigUint) -> Self {
                {% for field in entity.fields -%}
                    let {{ field.name }} = {{ field.type }}::decode(number);
                {% endfor -%}
                Self {
                    {% for field in entity.fields -%}
                        {{ field.name }},
                    {% endfor %}
                }
            }
        }
    {% elif entity.type == "enum" %}
        pub enum {{ entity.name }} {
            {% for variant in entity.variants -%}
                {{ variant.name }}(
                    {% if variant.associated_type != None %}
                        {{ variant.associated_type }}
                    {% endif %}
                ),
            {% endfor %}
        }

        impl {{ entity.name }} {
            #[allow(dead_code)]
            pub fn base() -> BigUint {
                BigUint::from({{ entity.variants|length }}usize)
            }
            pub fn encode(&self, number: &mut BigUint) {
                match self {
                    {% for variant in entity.variants -%}
                        Self::{{ variant.name }}(
                            {% if variant.associated_type != None %}
                                associated_value
                            {% endif %}
                        ) => {
                            {% if variant.associated_type != None %}
                                associated_value.encode(number);
                            {% endif -%}
                            write(number, &Self::base(), &BigUint::from({{ loop.index0 }}usize));
                        }
                    {% endfor %}
                }
            }
            pub fn decode(number: &mut BigUint) -> Self {
                let num = read(number, &Self::base());
                {% for variant in entity.variants -%}
                    if num == BigUint::from({{ loop.index0 }}usize) {
                        {% if variant.associated_type != None %}
                            let associated_value = {{ variant.associated_type }}::decode(number);
                        {% endif -%}
                        return Self::{{ variant.name }}(
                            {% if variant.associated_type != None %}
                                associated_value
                            {% endif %}
                        );
                    }
                {% endfor -%}
                unreachable!()
            }
        }
    {% elif entity.type == "sequence" %}
        pub struct {{ entity.name }} {
            pub items: Vec<{{ entity.content_type }}>,
        }

        impl {{ entity.name }} {
            #[allow(dead_code)]
            pub fn base() -> BigUint {
                {{ entity.content_type }}::base() + 1usize
            }
            pub fn encode(&self, number: &mut BigUint) {
                write(number, &Self::base(), &BigUint::ZERO);
                for item in self.items.iter().rev() {
                    item.encode(number);
                    let item = read(number, &{{ entity.content_type }}::base());
                    write(number, &Self::base(), &(item + 1usize));
                }
            }
            pub fn decode(number: &mut BigUint) -> Self {
                let mut items = Vec::new();
                loop {
                    let num = read(number, &Self::base());
                    if num == BigUint::ZERO {
                        break;
                    }
                    write(number, &{{ entity.content_type }}::base(), &(num - 1usize));
                    items.push({{ entity.content_type }}::decode(number));
                }
                Self { items }
            }
        }
    {% endif %}
{% endfor %}
""".strip())

def main():
    input_file_name = "entities.json"
    with open(input_file_name, "r", encoding="utf-8") as f:
        entities = json.load(f)

    with open("entities.rs", "w", encoding="utf-8") as f:
        f.write(TEMPLATE.render(
            entities=entities,
            input_file_name=input_file_name,
        ))

    os.system("rustfmt entities.rs")

main()
