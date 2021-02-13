#[derive(Debug)]
pub enum Error {
    XMLParseError(minidom::Error),
    URLNotFoundError,
    DecodeError(base64::DecodeError),
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::DecodeError(error)
    }
}

impl From<minidom::Error> for Error {
    fn from(error: minidom::Error) -> Self {
        Error::XMLParseError(error)
    }
}
