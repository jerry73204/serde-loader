use crate::{
    common::*,
    file::{FileDumper, FileLoader, FilePath},
};
use prost::Message;

pub type ProtobufPath<T> = FilePath<T, ProtobufDumper, ProtobufLoader>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtobufDumper {
    _private: [u8; 0],
}

impl<T> FileDumper<T> for ProtobufDumper
where
    T: Message,
{
    type Error = anyhow::Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let bytes = value.encode_to_vec();
        fs::write(p, &bytes)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtobufLoader {
    _private: [u8; 0],
}

impl<T> FileLoader<T> for ProtobufLoader
where
    T: Message + Default,
{
    type Error = anyhow::Error;

    fn load<P>(p: P) -> Result<T, Self::Error>
    where
        P: AsRef<Path>,
    {
        let bytes = fs::read(p)?;
        let value = T::decode(&*bytes)?;
        Ok(value)
    }
}
