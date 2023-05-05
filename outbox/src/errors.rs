use core::fmt;

#[derive(Debug)]
pub enum Error {
    ConnectionError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConnectionError => write!(f, "Database connection error"),
        }
    }
}
