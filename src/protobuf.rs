use crate::common::*;
use crate::file::FileDumper;
use crate::file::FileLoader;
use crate::file::FilePath;
use prost::Message;

pub type ProtobufPath<T> = FilePath<T, ProtobufDumper, ProtobufLoader>;

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
