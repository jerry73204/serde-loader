#![doc = r##"Serde wrapper to load/save serializable data from relative paths."##]
#![doc = r##"
This crate provides special wrappers to ser/deserialize type from a file path.
The following wrappers are provided:

"##]
#![cfg_attr(feature = "json", doc = "- [JsonPath]")]
#![cfg_attr(feature = "json", doc = "- [JsonPrettyPath]")]
#![cfg_attr(feature = "json5", doc = "- [Json5Path]")]
#![cfg_attr(feature = "protobuf", doc = "- [ProtobufPath]")]
#![doc = ""]
#![cfg_attr(
    feature = "json",
    doc = r##"
It enables to read or write the file path instead of data during de/serialization.
For example, we can have a `main.json` file that loads `external.json`.

`main.json`

```json
{
    "external": "external.json"
}
```

The type type definition is defined as follows.

```no_run
# fn main() -> anyhow::Result<()> {
use serde_loader::JsonPath;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Main {
    pub external: JsonPath<External>
}

#[derive(Serialize, Deserialize)]
struct External {
    data: String
}

// open file
let main: JsonPath<Main> = JsonPath::open("main.json")?;

// access data
let data = &main.external.data;
# Ok(())
# }
```

# Recursive File Loading

The file path is relative to the file where the field is defined.
It tracks file paths automatically and allows recursive file loading.

Suppose we have the following files.

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

Let's load the files recursively.


```rust
# fn main() -> anyhow::Result<()> {
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
# Ok(())
# }
```
"##
)]

mod common;
mod dir_stack;
pub mod error;

pub use abs_path_buf::AbsPathBuf;
pub mod abs_path_buf;

pub use file::FilePath;
pub mod file;

#[cfg(feature = "protobuf")]
pub use protobuf::ProtobufPath;
#[cfg(feature = "protobuf")]
mod protobuf;

#[cfg(feature = "json")]
pub use json::{JsonPath, JsonPrettyPath};
#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "json5")]
pub use self::json5::Json5Path;
#[cfg(feature = "json5")]
pub mod json5;
