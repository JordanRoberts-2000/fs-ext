mod created;
mod file_type;
mod last_modified;

pub use {
    crate::fsx::file::size, created::created, file_type::file_type, last_modified::last_modified,
};
