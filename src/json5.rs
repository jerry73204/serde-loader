use crate::{
    common::*,
    file::{FileDumper, FileLoader, FilePath},
};

pub type Json5Path<T> = FilePath<T, Json5Dumper, Json5Loader>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json5Dumper {
    _private: [u8; 0],
}

impl<T> FileDumper<T> for Json5Dumper
where
    T: Serialize,
{
    type Error = anyhow::Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
    {
        let text = json5::to_string(value)?;
        fs::write(p, &text)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Json5Loader {
    _private: [u8; 0],
}

impl<T> FileLoader<T> for Json5Loader
where
    T: DeserializeOwned,
{
    type Error = anyhow::Error;

    fn load<P>(p: P) -> Result<T, Self::Error>
    where
        P: AsRef<Path>,
    {
        let text = fs::read_to_string(p)?;
        let value = json5::from_str(&text)?;
        Ok(value)
    }
}
