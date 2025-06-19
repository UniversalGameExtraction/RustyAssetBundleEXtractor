mod de;
mod error;
mod ser;

pub use de::{from_reader, from_slice, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_vec, to_vec_endianed, to_writer, Serializer};
