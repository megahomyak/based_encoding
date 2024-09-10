import json
import os

with open("entities.json", "r", encoding="utf-8") as f:
    entities = json.load(f)

with open("entities.rs", "w", encoding="utf-8") as f:
    # Prelude
    f.write("use crate::number::{read, write, Base, Digit};")
    f.write("use num_bigint::BigUint;")

    for entity in entities:
        if entity["type"] == "sequence":
            pass # TODO

os.system("cargo fmt -- entities.rs")
