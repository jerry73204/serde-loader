use crate::{
    common::*,
    file::{FileDumper, FileLoader, FilePath},
};
use fs::File;
use io::{BufReader, BufWriter};

pub type JsonPath<T> = FilePath<T, JsonDumper, JsonLoader>;
pub type JsonPrettyPath<T> = FilePath<T, JsonDumperPretty, JsonLoader>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonDumper {
    _private: [u8; 0],
}

impl<T> FileDumper<T> for JsonDumper
where
    T: Serialize,
{
    type Error = anyhow::Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let mut writer = BufWriter::new(File::create(p)?);
        serde_json::to_writer(&mut writer, value)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonDumperPretty {
    _private: [u8; 0],
}

impl<T> FileDumper<T> for JsonDumperPretty
where
    T: Serialize,
{
    type Error = anyhow::Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let mut writer = BufWriter::new(File::create(p)?);
        serde_json::to_writer_pretty(&mut writer, value)?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsonLoader {
    _private: [u8; 0],
}

impl<T> FileLoader<T> for JsonLoader
where
    T: DeserializeOwned,
{
    type Error = anyhow::Error;

    fn load<P>(p: P) -> Result<T, Self::Error>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(p)?);
        let value = serde_json::from_reader(reader)?;
        Ok(value)
    }
}
