pub mod classes;

mod class_id;
pub mod pptr;

pub use class_id::ClassId;
pub use pptr::{PPtr, TypedPPtr};

pub trait ClassIdType {
    const CLASS_ID: ClassId;
}
