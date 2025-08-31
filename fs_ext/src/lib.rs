#[cfg(test)]
pub mod test_utils;

mod error;
pub mod formats;
pub mod fsx;
mod macros;
mod tokio_fsx;
mod traits;
pub(crate) mod utils;

pub mod tokio {
    pub mod fsx {
        pub use crate::tokio_fsx::*;
    }
}

pub use {
    error::{CodecError, DeserializeError, SerializeError},
    traits::{Format, IoResultExt},
};
