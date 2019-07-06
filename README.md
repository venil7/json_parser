# json-parser

JSON tokenizer and parser. JSON format supports following types:

- `Number`, translates to Rust `f68`
- `String`, translates to Rust owned `String`
- Null, doesnt have a direct translation in Rust, indicates absence of value
- Array, roughly tranlates to `Vec<T>`, where `T` is one of the listed types
- Object, roughly translates to `HashMap<String, T>`, where `T` is one of the listed types

## JSON value

is defined as follows:

```
pub enum Json {
 Null,
 Bool(bool),
 Number(f64),
 String(String),
 Array(Vec<Box<Json>>),
 Object(HashMap<String, Box<Json>>),
}
```

This enum implements `FromStr` trait and therefore can be used as follows:

```
let value: Json = json_string.parse::<Json>()?;
```

## install

Add to your `Cargo.toml`

```
json-parser = "1"
```
