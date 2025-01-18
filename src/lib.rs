mod de;
mod ser;

pub use ser::{Serializer, to_string};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug,Clone)]
pub enum Error {
    Message(String),
    NotSupported(&'static str),
    UnexpectedSequence(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => f.write_str(msg),
            Error::NotSupported(thing) => {
                f.write_str(thing)?;
                f.write_str(" is not supported (yet)")
            },
            Error::UnexpectedSequence(msg) => {
                f.write_str("Unexpected Sequence: \n")?;
                f.write_str(msg)
            }
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Error::Message(msg.to_string())
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg:T) -> Self where T:std::fmt::Display {
        Error::Message(msg.to_string())
    }
}

impl std::error::Error for Error {}

