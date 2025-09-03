# 📚 fs-ext — API Reference

## 🚀 Macros

- `file!(path, optional-content)` — Ensure or overwrite a file
- `dir!` — Ensure a directory exists
- `load!` — Typed load with extension inference
- `save!` — Typed save with extension inference
- `tempfile!` — Create temp file
- `tempdir!` — Create temp dir

See: `docs/macros.md`

---

## 🧩 Traits

- `IoResultExt` — Add to `io::Result<T>`, eg: `.with_path_context(...)`.
- `PathExt` — Strict checks/assertions on `Path` (`is_*_strict`, `assert_*`, `kind()`).
- `Format` — Pluggable (de)serialization with provided `load` / `save` helpers.

See: `docs/traits.md`

---

## 📦 Structs

- `TempDir` — RAII temp directory with `path()`, `keep()`, `close()`.
- `TempFile` — RAII temp file with `as_file[_mut]()`, `persist[_new]()`, `keep()`, `copy_from()`.
- `DirQuery` — Builder for directory scanning (`collect`, `count`, `exists`), with filters.

See: `docs/dirquery.md`

---

## 📁 Dir module — `fsx::dir`

### Checks

- `assert_exists(path)` — Must exist.
- `assert_not_exists(path)` — Must not exist.
- `exists(path) -> io::Result<bool>` — True if path is a directory.
- `is_empty(path) -> io::Result<bool>` — Directory has no entries.
- `size(path) -> io::Result<u64>` — Total size of regular files under dir (recursive).

### Creation

- `create_new(path)` — Create directory; error if exists.
- `ensure(path)` — Create (parents too) if missing.

### Query (via `DirQuery`)

- `entries(path) -> DirQuery` — Files + dirs, recursive by default.
- `files(path) -> DirQuery` — Only files.
- `subdirs(path) -> DirQuery` — Only directories.

### Temp

- `temp() -> TempDir` — Temp dir in system temp.
- `temp_in(parent) -> TempDir` — Temp dir under `parent`.

### Utils

- `clear(path)` — Remove all contents, keep the directory.
- `copy_contents(src, dst)` — Copy entries from `src` into existing `dst`.
- `copy(src, dst)` — Ensure `dst`, then copy entire tree.

See: `docs/dir.md`

---

## 📄 File module — `fsx::file`

### Checks

- `assert_exists(path)`
- `assert_not_exists(path)`
- `assert_readable(path)`
- `assert_writable(path)`
- `exists(path) -> io::Result<bool>`
- `is_empty(path) -> io::Result<bool>`
- `is_readable(path) -> io::Result<bool>`
- `is_writable(path) -> io::Result<bool>`
- `size(path) -> io::Result<u64>`

### Creation

- `create_new(path) -> File` — Error if exists.
- `ensure_or_init_with(path, || bytes) -> File`
- `ensure_or_init(path, bytes) -> File`
- `ensure(path) -> File` — Create if missing; don’t clobber.
- `overwrite(path) -> File` — Truncate to zero.
- `touch(path)` — Create if missing; bump mtime if present.

### Loading (typed)

- `load_auto<T: Deserialize>(path) -> Result<T, CodecError>` — Codec by extension.
- `load_or_default<T: Default + Deserialize>(path) -> Result<T, CodecError>`
- `load_or_init_with<T: Serialize + Deserialize>(path, || value) -> Result<T, CodecError>`
- `load_or_init<T: Serialize + Deserialize>(path, value) -> Result<T, CodecError>`
- `load_or_write_str(path, &str) -> io::Result<String>`
- `load<T, F: Format>(path) -> Result<T, CodecError>` — Explicit codec.

### Meta

- `meta::created(path) -> io::Result<SystemTime>`
- `meta::file_type(path) -> io::Result<std::fs::FileType>`
- `meta::last_modified(path) -> io::Result<SystemTime>`

### Misc

- `append(path) -> io::Result<File>` — Open for append (create if missing).
- `open(path) -> io::Result<File>` — Open read/write without truncate.

### Reading

- `read_bytes(path) -> io::Result<Vec<u8>>`
- `read_lines(path) -> io::Result<impl Iterator<Item = io::Result<String>>>`
- `read_string_or_init_with(path, || bytes) -> io::Result<String>`
- `read_string_or_init(path, bytes) -> io::Result<String>`
- `read_string(path) -> io::Result<String>`

### Saving (typed & atomic)

- `save<T, F: Format>(path, model: T) -> Result<(), CodecError>` — **Explicit codec** (ignores extension).
- `save_auto<T: Serialize>(path, &model) -> Result<(), CodecError>` — **Codec from extension** (`json`, `toml`, `yaml/yml`).

### Streaming

- `stream_bytes(path) -> io::Result<impl Read>`
- `stream_lines(path) -> io::Result<impl Iterator<Item = io::Result<String>>>`

### Atomic

- `atomic::create_new(path, |f| ...)` — Fail if `path` exists.
- `atomic::overwrite(path, |f| ...)` — Replace/create atomically.
- `atomic::update(path, |current: &str, f| ...)` — Read-modify-write atomically.

### Open (modes)

- `open::write_only(path) -> io::Result<File>`
- `open::read_only(path) -> io::Result<File>`

### Removal

- `remove(path)` — Permanent delete.
- `trash(path)` — Move to system trash (if supported).
- `trash_or_remove(path)` — Prefer trash; fallback to delete.

### Temp

- `temp() -> TempFile`
- `temp_in(parent) -> TempFile`

See: `docs/file.md`

---
