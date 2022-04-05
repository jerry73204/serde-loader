use crate::{
    common::*,
    dir_stack::{try_rebase_path, with_rebased_dir},
    error::{Error, FileDumpError, FileLoadError},
};

/// A wrapper type that opens and deserializes a JSON file to type `T`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePath<T, Ser, De> {
    inner: Option<Inner<T>>,
    _phantom: PhantomData<(Ser, De)>,
}

/// A wrapper type that opens and deserializes a JSON file to type `T`.
#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, Hash)]
struct Inner<T> {
    data: T,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    ref_path: PathBuf,
    #[derivative(PartialEq = "ignore", Hash = "ignore")]
    abs_path: PathBuf,
}

impl<T, Ser, De> FilePath<T, Ser, De>
where
    De: FileLoader<T>,
{
    pub fn open<P>(path: P) -> Result<Self, Error<Infallible, De::Error>>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        let ref_path = path.as_ref();
        let abs_path = try_rebase_path(ref_path).into_owned();

        let parent = abs_path
            .parent()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format_err!(
                        "unable to find parent directory of '{}'",
                        abs_path.display()
                    ),
                )
            })?
            .to_owned();

        let data = with_rebased_dir(parent, || {
            De::load(&abs_path).map_err(|error| FileLoadError {
                path: abs_path.clone(),
                error,
            })
        })?;

        Ok(Self {
            inner: Some(Inner {
                data,
                ref_path: ref_path.to_path_buf(),
                abs_path,
            }),
            _phantom: PhantomData,
        })
    }

    pub fn open_and_take<P>(path: P) -> Result<T, Error<Infallible, De::Error>>
    where
        P: AsRef<Path>,
        T: DeserializeOwned,
    {
        Ok(Self::open(path)?.take())
    }
}

impl<T, Ser, De> FilePath<T, Ser, De>
where
    Ser: FileDumper<T>,
{
    pub fn save(&self) -> Result<(), Error<Ser::Error, Infallible>>
    where
        T: Serialize,
    {
        let abs_path = &self.inner().abs_path;
        let parent = abs_path
            .parent()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format_err!(
                        "unable to find parent directory of '{}'",
                        abs_path.display()
                    ),
                )
            })?
            .to_owned();

        with_rebased_dir(parent, || {
            Ser::dump(&abs_path, &self.inner().data).map_err(|error| FileDumpError {
                error,
                path: abs_path.to_path_buf(),
            })
        })?;

        Ok(())
    }
}

impl<T, Ser, De> FilePath<T, Ser, De> {
    pub fn take(mut self) -> T {
        self.inner.take().unwrap().data
    }

    pub fn abs_path(&self) -> &Path {
        &self.inner().abs_path
    }

    pub fn ref_path(&self) -> &Path {
        &self.inner().ref_path
    }

    fn inner(&self) -> &Inner<T> {
        self.inner.as_ref().unwrap()
    }

    fn inner_mut(&mut self) -> &mut Inner<T> {
        self.inner.as_mut().unwrap()
    }
}

impl<T, Ser, De> AsRef<T> for FilePath<T, Ser, De> {
    fn as_ref(&self) -> &T {
        &self.inner().data
    }
}

impl<T, Ser, De> Deref for FilePath<T, Ser, De> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner().data
    }
}

impl<T, Ser, De> DerefMut for FilePath<T, Ser, De> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_mut().data
    }
}

impl<T, Ser, De> Serialize for FilePath<T, Ser, De>
where
    T: Serialize,
    Ser: FileDumper<T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.save().map_err(S::Error::custom)?;
        self.inner().ref_path.serialize(serializer)
    }
}

impl<'de, T, Ser, De> Deserialize<'de> for FilePath<T, Ser, De>
where
    T: DeserializeOwned,
    De: FileLoader<T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ref_path = PathBuf::deserialize(deserializer)?;
        Self::open(ref_path).map_err(D::Error::custom)
    }
}

pub trait FileLoader<T>
where
    Self::Error: fmt::Display,
{
    type Error;

    fn load<P>(p: P) -> Result<T, Self::Error>
    where
        P: AsRef<Path>;
}

pub trait FileDumper<T>
where
    Self::Error: fmt::Display,
{
    type Error;

    fn dump<P>(p: P, value: &T) -> Result<(), Self::Error>
    where
        P: AsRef<Path>;
}
