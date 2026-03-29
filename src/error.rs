use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    IOError(std::io::Error),
    SerializeTomlError(toml::ser::Error),
    DeserializeTomlError(toml::de::Error),
    SystemTimeError(std::time::SystemTimeError),
    SQLError(rusqlite::Error),
    ConfigError(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IO Error - {}", e),
            Error::SerializeTomlError(e) => write!(f, "Serialize TOML Error - {}", e),
            Error::DeserializeTomlError(e) => write!(f, "Deserialize TOML Error - {}", e),
            Error::SystemTimeError(e) => write!(f, "System Time Error - {}", e),
            Error::SQLError(e) => write!(f, "SQL Error - {}", e),
            Error::ConfigError(msg) => write!(f, "Config Error - {}", msg),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IO Error - {}", e),
            Error::SerializeTomlError(e) => write!(f, "Serialize TOML Error - {}", e),
            Error::DeserializeTomlError(e) => write!(f, "Deserialize TOML Error - {}", e),
            Error::SystemTimeError(e) => write!(f, "System Time Error - {}", e),
            Error::SQLError(e) => write!(f, "SQL Error - {}", e),
            Error::ConfigError(msg) => write!(f, "Config Error - {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(e) => Some(e),
            Error::SerializeTomlError(e) => Some(e),
            Error::DeserializeTomlError(e) => Some(e),
            Error::SystemTimeError(e) => Some(e),
            Error::SQLError(e) => Some(e),
            _ => None,
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

impl From<std::time::SystemTimeError> for Error {
    fn from(err: std::time::SystemTimeError) -> Self {
        Error::SystemTimeError(err)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::SQLError(err)
    }
}
