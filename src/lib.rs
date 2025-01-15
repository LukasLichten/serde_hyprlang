mod de;
mod ser;

pub enum Error {
    Other(String)
}

pub type Result<T> = std::result::Result<T, Error>;
