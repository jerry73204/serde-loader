#![cfg(feature = "json5")]

use anyhow::ensure;
use serde::{Deserialize, Serialize};
use serde_loader::Json5Path;
use std::fs;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Data {
    pub key: u32,
    pub value: String,
}

const CREATE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/create_test.json5");
const SAVE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/save_test.json5");

#[test]
fn json5_create() {
    let path = CREATE_PATH;

    let result = (move || {
        let orig_data = {
            let data = Data {
                key: 7,
                value: "wee".into(),
            };
            let data = Json5Path::create(path, data)?;
            data.take()
        };

        {
            let data: Data = Json5Path::open_and_take(path)?;
            dbg!(&data, &orig_data);
            ensure!(data == orig_data);
        }

        Ok(())
    })();

    let _ = fs::remove_file(path);
    result.unwrap();
}

#[test]
fn json5_save() {
    let path = SAVE_PATH;

    let result = (move || {
        let orig_data = {
            let data = Data {
                key: 7,
                value: "wee".into(),
            };
            let mut data = Json5Path::create(path, data)?;
            data.key = 11;
            data.save()?;
            data.take()
        };

        {
            let data: Data = Json5Path::open_and_take(path)?;
            ensure!(data == orig_data);
        }

        Ok(())
    })();

    let _ = fs::remove_file(path);
    result.unwrap();
}
