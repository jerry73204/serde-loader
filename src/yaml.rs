use crate::{
    file::{FileDumper, FileLoader},
    FilePath,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

pub type YamlPath<T> = FilePath<T, YamlDumper, YamlLoader>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YamlDumper {
    _private: [u8; 0],
}

impl<T> FileDumper<T> for YamlDumper
where
    T: Serialize,
{
    type Error = anyhow::Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let writer = BufWriter::new(File::create(p)?);
        serde_yaml::to_writer(writer, value)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YamlLoader {
    _private: [u8; 0],
}

impl<T> FileLoader<T> for YamlLoader
where
    T: for<'de> Deserialize<'de>,
{
    type Error = anyhow::Error;

    fn load<P>(p: P) -> Result<T, Self::Error>
    where
        P: AsRef<Path>,
    {
        let reader = BufReader::new(File::open(p)?);
        let value: T = serde_yaml::from_reader(reader)?;
        Ok(value)
    }
}
