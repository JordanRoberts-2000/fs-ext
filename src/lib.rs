mod error;
pub mod formats;
pub mod fsx;
mod tokio_fsx;
mod traits;

pub mod tokio {
    pub mod fsx {
        pub use crate::tokio_fsx::*;
    }
}

pub use {
    error::{CodecError, DeserializeError, SerializeError},
    traits::Format,
};
