pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    err: Box<dyn std::error::Error>
}

impl std::error::Error for Error{}

impl std::fmt::Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error{err: Box::new(e)}
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error{err: Box::new(e)}
    }
}