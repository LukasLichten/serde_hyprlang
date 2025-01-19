use serde::Deserialize;

use crate::{Error, Result};


pub fn from_str<'a, T>(_text: &'a str) -> Result<T> where T: Deserialize<'a> {
    Err(Error::NotSupported("Deserialization"))
} 
