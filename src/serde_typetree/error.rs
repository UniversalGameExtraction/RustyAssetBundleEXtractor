pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Error(Box<ErrorImpl>);

impl Error {
    pub(crate) fn new(error: ErrorImpl) -> Self {
        Error(Box::new(error))
    }

    pub fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error(Box::new(ErrorImpl::Custom(msg.to_string())))
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub(crate) enum ErrorImpl {
    IO(std::io::Error),
    Custom(String),
    Enum(&'static str),
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0 {
            ErrorImpl::Custom(msg) => msg.fmt(f),
            ErrorImpl::IO(error) => error.fmt(f),
            ErrorImpl::Enum(name) => {
                write!(
                    f,
                    "enum {name}: serde_typetree only supports #[serde(untagged)] enums"
                )
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error(Box::new(ErrorImpl::IO(value)))
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::custom(msg)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::custom(msg)
    }
}
