use crate::common::*;

#[derive(Debug)]
pub enum Error<E1 = Infallible, E2 = Infallible> {
    Io(io::Error),
    FileDump(FileDumpError<E1>),
    FileLoad(FileLoadError<E2>),
}

impl<E1, E2> fmt::Display for Error<E1, E2>
where
    E1: fmt::Display,
    E2: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "{}", err),
            Error::FileDump(err) => write!(f, "{}", err),
            Error::FileLoad(err) => write!(f, "{}", err),
        }
    }
}

impl<E1, E2> From<FileLoadError<E2>> for Error<E1, E2> {
    fn from(v: FileLoadError<E2>) -> Self {
        Self::FileLoad(v)
    }
}

impl<E1, E2> From<FileDumpError<E1>> for Error<E1, E2> {
    fn from(v: FileDumpError<E1>) -> Self {
        Self::FileDump(v)
    }
}

impl<E1, E2> From<io::Error> for Error<E1, E2> {
    fn from(v: io::Error) -> Self {
        Self::Io(v)
    }
}

#[derive(Debug)]
pub struct FileLoadError<E> {
    pub path: PathBuf,
    pub error: E,
}

impl<E> StdError for FileLoadError<E> where E: StdError {}

impl<E> fmt::Display for FileLoadError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unable to load file '{}'\n{}",
            self.path.display(),
            self.error
        )
    }
}

#[derive(Debug)]
pub struct FileDumpError<E> {
    pub path: PathBuf,
    pub error: E,
}

impl<E> StdError for FileDumpError<E> where E: StdError {}

impl<E> fmt::Display for FileDumpError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unable to save to file '{}'\n{}",
            self.path.display(),
            self.error
        )
    }
}
