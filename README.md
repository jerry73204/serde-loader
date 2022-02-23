# serde-loader

\[ [crates.io](https://crates.io/crates/serde-loader/) | [docs.rs](https://docs.rs/serde-load/) \]

It provides [serde](https://docs.rs/serde/) wrapper to load/save serializable data from relative paths.

## Example

It allows to write file paths instead of data during serialization.
Suppose we have the following JSON files to be loaded.

`main.json`

```json
{
    "sub": "sub/sub.json"
}
```

`sub/sub.json`

```json
{
    "sub": "sub/sub.json"
}
```

`sub/sub/sub_of_sub.json`

```json
{
    "sub": "sub/sub_of_sub.json"
}
```

This crate provides the `JsonPath` wrapper load JSON files recursively.

```rust
use serde_loader::JsonPath;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Main {
    pub sub: JsonPath<Sub>
}


#[derive(Serialize, Deserialize)]
struct Sub {
    pub sub: JsonPath<SubOfSub>
}


#[derive(Serialize, Deserialize)]
struct SubOfSub {
    pub name: String,
    pub value: String,
}

let config: JsonPath<Main> = JsonPath::open("tests/config-example/main.json")?;
config.save()?;
```


## License

MIT license. See the [license file](LICENSE.txt).
