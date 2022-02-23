use crate::{common::*, dir_stack::try_rebase_path};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AbsPathBuf(PathBuf);

impl AbsPathBuf {
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    pub fn into_inner(self) -> PathBuf {
        self.0
    }
}

impl TryFrom<&'_ OsStr> for AbsPathBuf {
    type Error = anyhow::Error;

    fn try_from(from: &'_ OsStr) -> Result<Self, Self::Error> {
        let path = Path::new(from);
        let path = try_rebase_path(path);
        Ok(Self(path.into_owned()))
    }
}

impl TryFrom<&'_ Path> for AbsPathBuf {
    type Error = anyhow::Error;

    fn try_from(from: &'_ Path) -> Result<Self, Self::Error> {
        Self::try_from(from.as_os_str())
    }
}

impl TryFrom<&'_ PathBuf> for AbsPathBuf {
    type Error = anyhow::Error;

    fn try_from(from: &PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(from.as_os_str())
    }
}

impl TryFrom<PathBuf> for AbsPathBuf {
    type Error = anyhow::Error;

    fn try_from(from: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(from.as_os_str())
    }
}

impl AsRef<Path> for AbsPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Deref for AbsPathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Serialize for AbsPathBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let path = &self.0;
        assert!(path.is_absolute(), "please report bug");

        path.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AbsPathBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        let path = Self::try_from(&path).map_err(|_| {
            D::Error::custom(format!(
                "unable to resolve the absolute path of '{}'",
                path.display()
            ))
        })?;

        Ok(path)
    }
}
