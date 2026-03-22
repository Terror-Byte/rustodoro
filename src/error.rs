use core::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerializeTomlError(toml::ser::Error),
    DeserializeTomlError(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "{}", e),
            Error::SerializeTomlError(e) => write!(f, "{}", e),
            Error::DeserializeTomlError(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::SerializeTomlError(e) => Some(e),
            Error::DeserializeTomlError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::SerializeTomlError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::DeserializeTomlError(err)
    }
}
