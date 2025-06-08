mod de;
mod error;

pub use de::{from_reader, from_slice, Deserializer};
pub use error::{Error, Result};
