//! [serde] support for unity typetrees.
mod de;
mod error;
mod ser;

pub use de::{Deserializer, from_reader, from_slice};
pub use error::{Error, Result};
pub use ser::{Serializer, to_vec, to_vec_endianed, to_writer};
