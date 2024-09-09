import json

with open("entities.json", encoding="utf-8") as f:
    entities_input = json.load(f)

# sequence<T>
# struct<A, B, C, ...>
# enum (numbers)
#   encoding:
#       given value -> based digit
#   decoding:
#       just read the based digit

with open("entities.rs", "w", encoding="utf-8") as f:
    def w(a):
        f.write(a)
    for entity in entities_input:
        if entity["type"] == "enum":
            for field in entity["fields"]:
                w(f"")
                field["name"]
