Macros:
file!
dir!
load!
save!
temp!

Traits:
IoResultExt

Structs:
TempDir
TempFile
DirQuery

Dir module:
-- checks
fsx::dir::assert_exists
fsx::dir::assert_not_exists
fsx::dir::exists
fsx::dir::is_empty
fsx::dir::size
-- creation
fsx::dir::create_new
fsx::dir::ensure
-- query
fsx::dir::entries
fsx::dir::files
fsx::dir::subdirs
-- temp
fsx::dir::temp
fsx::dir::temp_in
-- utils
fsx::dir::clear
fsx::dir::copy_contents
fsx::dir::copy

File module:
-- checks
fsx::file::assert_exists
fsx::file::assert_not_exists
fsx::file::assert_readable
fsx::file::assert_writable
fsx::file::exists
fsx::file::is_empty
fsx::file::is_readable
fsx::file::is_writable
fsx::file::size
-- creation
fsx::file::create_new
fsx::file::ensure_or_init_with
fsx::file::ensure_or_init
fsx::file::ensure
fsx::file::overwrite
fsx::file::touch
-- loading
fsx::file::load_auto
fsx::file::load_or_default
fsx::file::load_or_init_with
fsx::file::load_or_init
fsx::file::load_or_write_str
fsx::file::load
-- meta submodule
fsx::file::meta::created
fsx::file::meta::file_type
fsx::file::meta::last_modified
-- misc
fsx::file::append
fsx::file::open
-- reading
fsx::file::read_bytes
fsx::file::read_lines
fsx::file::read_string_or_init_with
fsx::file::read_string_or_init
fsx::file::read_string
-- saving
fsx::file::save
fsx::file::save_auto
-- streaming
fsx::file::stream_bytes
fsx::file::stream_lines
-- atomic submodule
fsx::file::atomic::create_new
fsx::file::atomic::overwrite
fsx::file::atomic::update
-- open submodule
fsx::file::open::write_only
fsx::file::open::read_only
-- removal
fsx::file::remove
fsx::file::trash
fsx::file::trash_or_remove
-- temp
fsx::file::temp
fsx::file::temp_in
