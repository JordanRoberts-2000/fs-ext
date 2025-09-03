# 📄 `fsx::file` — File Utilities

Utilities for validating, creating, opening, reading/writing, and atomically updating files.  
Designed to give clear errors (with path context) and safe defaults (e.g., atomic writes).

---

## 📜 Index

- **Checks**
  - [`assert_exists`](#assert_exists)
  - [`assert_not_exists`](#assert_not_exists)
  - [`assert_readable`](#assert_readable)
  - [`assert_writable`](#assert_writable)
  - [`exists`](#exists)
  - [`is_empty`](#is_empty)
  - [`is_readable`](#is_readable)
  - [`is_writable`](#is_writable)
  - [`size`](#size)
- **Creation**
  - [`create_new`](#create_new)
  - [`ensure`](#ensure)
  - [`ensure_or_init`](#ensure_or_init)
  - [`ensure_or_init_with`](#ensure_or_init_with)
  - [`overwrite`](#overwrite)
  - [`touch`](#touch)
- **Loading**
  - [`load`](#load)
  - [`load_auto`](#load_auto)
  - [`load_or_default`](#load_or_default)
  - [`load_or_init_with`](#load_or_init_with)
  - [`load_or_init`](#load_or_init)
  - [`load_or_write_str`](#load_or_write_str)
- **Meta**
  - [`meta::created`](#metacreated)
  - [`meta::file_type`](#metadatafile_type)
  - [`meta::last_modified`](#metalast_modified)
- **Misc**
  - [`append`](#append)
  - [`open`](#open)
- **Reading**
  - [`read_bytes`](#read_bytes)
  - [`read_lines`](#read_lines)
  - [`read_string_or_init_with`](#read_string_or_init_with)
  - [`read_string_or_init`](#read_string_or_init)
  - [`read_string`](#read_string)
- **Saving**
  - [`save`](#save)
  - [`save_auto`](#save_auto)
- **Streaming**
  - [`stream_bytes`](#stream_bytes)
  - [`stream_lines`](#stream_lines)
- **Atomic**
  - [`atomic::create_new`](#atomiccreate_new)
  - [`atomic::overwrite`](#atomicoverwrite)
  - [`atomic::update`](#atomicupdate)
- **Open (modes)**
  - [`open::write_only`](#openwrite_only)
  - [`open::read_only`](#openread_only)
- **Removal**
  - [`remove`](#remove)
  - [`trash`](#trash)
  - [`trash_or_remove`](#trash_or_remove)
- **Temp**
  - [`temp`](#temp)
  - [`temp_in`](#temp_in)

---

## 🔎 Checks

### `assert_exists`

Ensure the path exists and is a **regular file**.

```rust
use fs_ext::fsx::file;
file::assert_exists("app.log")?;
```

---

### `assert_not_exists`

Ensure no file exists at path.

```rust
use fs_ext::fsx::file;
file::assert_not_exists("newfile.txt")?;
```

---

### `assert_readable`

Ensure the file exists and can be opened for reading.

```rust
use fs_ext::fsx::file;
file::assert_readable("config.toml")?;
```

---

### `assert_writable`

Ensure the file can be opened for writing.

```rust
use fs_ext::fsx::file;
file::assert_writable("output.bin")?;
```

---

### `exists`

Return whether a path exists and is a **file**.

```rust
use fs_ext::fsx::file;
if file::exists("CHANGELOG.md")? { /* ... */ }
```

---

### `is_empty`

Return `true` if file length is 0.

```rust
use fs_ext::fsx::file;
if file::is_empty("cache.dat")? { /* seed it */ }
```

---

### `is_readable` / `is_writable`

Heuristic checks that attempt to open the file with the corresponding mode.

```rust
use fs_ext::fsx::file;
let can_read = file::is_readable("notes.txt")?;
let can_write = file::is_writable("notes.txt")?;
```

---

### `size`

Return file length in bytes.

```rust
use fs_ext::fsx::file;
let bytes = file::size("image.png")?;
```

---

## 🧱 Creation

### `create_new`

Create a **new** file; fails if it already exists.

```rust
use fs_ext::fsx::file;
let f = file::create_new("fresh.txt")?;
```

---

### `ensure_or_init_with`

If missing, create file and write content from a closure; otherwise leave as is.

```rust
use fs_ext::fsx::file;
let _f = file::ensure_or_init_with("settings.json", || b"{\"first\":true}" as &[u8])?;
```

---

### `ensure_or_init`

Same as above but with a static buffer.

```rust
use fs_ext::fsx::file;
let _f = file::ensure_or_init("readme.txt", b"Hello\n")?;
```

---

### `ensure`

Create if missing.

```rust
use fs_ext::fsx::file;
let _f = file::ensure("db.sqlite3")?;
```

---

### `overwrite`

Open for write and truncate to zero. Returns `File`.

```rust
use fs_ext::fsx::file;
use std::io::Write;
let mut f = file::overwrite("output.log")?;
writeln!(f, "fresh start")?;
```

---

### `touch`

Create the file if missing; update mtime if it exists.

```rust
use fs_ext::fsx::file;
file::touch("touch_me")?;
```

---

## 📦 Loading (typed)

> These work with your crate’s `Format` implementations.  
> `*_auto` variants infer the format from extension (e.g., `.json`, `.toml`, `.yaml`).

### `load_auto`

Deserialize from a path by inferring the format from extension.

```rust
use serde::Deserialize;
use fs_ext::fsx::file;

#[derive(Deserialize)]
struct Cfg { name: String }

let cfg: Cfg = file::load_auto("config.json")?;
```

---

### `load_or_default`

If the file is missing or empty, return `T::default()`; otherwise load.

```rust
use serde::Deserialize;
#[derive(Deserialize, Default)]
struct Cfg { name: String }
let cfg: Cfg = fs_ext::fsx::file::load_or_default("cfg.toml")?;
```

---

### `load_or_init_with`

If missing, create by serializing the value returned from a closure; then load.

```rust
let cfg = fs_ext::fsx::file::load_or_init_with("prefs.json", || serde_json::json!({"dark":true}))?;
```

---

### `load_or_init`

Like above but with a ready value.

```rust
use serde::Serialize;
#[derive(Serialize)]
struct Pref { dark: bool }
let cfg = fs_ext::fsx::file::load_or_init("prefs.json", Pref { dark: true })?;
```

---

### `load_or_write_str`

If missing, write the provided string and then read it back (stringly).

```rust
let s = fs_ext::fsx::file::load_or_write_str("notes.txt", "hello\n")?;
```

---

### `load`

Deserialize using an explicit `Format` type parameter (no extension inference).

```rust
use serde::Deserialize;
#[derive(Deserialize)]
struct Cfg { name: String }
// e.g., Json::load::<Cfg>("path")
```

---

## 🧾 Meta

### `meta::created`

Return creation time.

```rust
let t = fs_ext::fsx::file::meta::created("out.log")?;
```

---

### `meta::file_type`

Return a platform `FileType`.

```rust
let ty = fs_ext::fsx::file::meta::file_type("archive.tar")?;
```

---

### `meta::last_modified`

Return the last modified time.

```rust
let t = fs_ext::fsx::file::meta::last_modified("data.csv")?;
```

---

## 🧰 Misc

### `append`

Append to a file (fails if missing).

```rust
use fs_ext::fsx::file;
file::append("app.log", b"additional content")?;
```

---

### `open`

Open read/write without truncation (fails if missing).

```rust
let f = fs_ext::fsx::file::open("db.sqlite3")?;
```

---

## 📖 Reading

### `read_bytes`

Read the entire file into `Vec<u8>`.

```rust
let bytes = fs_ext::fsx::file::read_bytes("image.png")?;
```

---

### `read_lines`

Read an iterator over lines.

```rust
for line in fs_ext::fsx::file::read_lines("README.md")? {
    let line = line?;
    // ...
}
```

---

### `read_string_or_init_with`

If missing, create by writing bytes from a closure, then read to `String`.

```rust
let s = fs_ext::fsx::file::read_string_or_init_with("hello.txt", || "hi".as_bytes())?;
```

---

### `read_string_or_init`

If missing, create by writing the given bytes, then read to `String`.

```rust
let s = fs_ext::fsx::file::read_string_or_init("hello.txt", b"hi")?;
```

---

### `read_string`

Read entire file into `String`.

```rust
let s = fs_ext::fsx::file::read_string("Cargo.toml")?;
```

---

## 💾 Saving

Two ways to write typed data (both are **atomic**: temp file → replace):

### `save`

Pick the **format explicitly** via the `Format` trait.

```rust
use fs_ext::{fsx::file, formats::Json};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Cfg { name: String, debug: bool }

file::save::<Cfg, Json>("settings.json", Cfg { name: "demo".into(), debug: true })?;
```

---

### `save_auto`

Infer the **format from the extension** (`.json`, `.toml`, `.yaml/.yml`).

```rust
use fs_ext::fsx::file;
use serde::Serialize;

#[derive(Serialize)]
struct Cfg { name: String }

let cfg = Cfg { name: "demo".into() };

file::save_auto("config.json", &cfg)?;  // → Json
file::save_auto("config.toml", &cfg)?;  // → Toml
```

---

## 🌊 Streaming

> Useful for large files or progressive consumption/production.

### `stream_bytes`

Return a byte stream reader (e.g., `Read`/iterator) for incremental processing.

```rust
let mut reader = fs_ext::fsx::file::stream_bytes("large.bin")?;
```

---

### `stream_lines`

Return a line-streaming reader to process large text files lazily.

```rust
let lines = fs_ext::fsx::file::stream_lines("access.log")?;
for line in lines {
    let line = line?;
    // ...
}
```

---

## 🧨 Atomic

> Atomic operations write to a temporary file and then atomically replace the destination to avoid torn writes.

### `atomic::create_new`

Create a brand-new destination by writing via a temp file; fails if `dst` exists.

```rust
use std::io::Write;
fs_ext::fsx::file::atomic::create_new("report.txt", |f| {
    writeln!(f, "report v1")?;
    Ok(())
})?;
```

---

### `atomic::overwrite`

Always replace destination with the data written in the closure.

```rust
use std::io::Write;
fs_ext::fsx::file::atomic::overwrite("state.json", |f| {
    f.write_all(br#"{"ok":true}"#)
})?;
```

---

### `atomic::update`

Read existing file, compute an update, and write atomically.

```rust
use std::io::Write;
// pseudo: reads current then lets you write the new version
fs_ext::fsx::file::atomic::update("counter.txt", |current, f| {
    let n: u64 = current.trim().parse().unwrap_or(0);
    write!(f, "{}", n + 1)?;
    Ok(())
})?;
```

---

## ✍️ Open (modes)

### `open::write_only`

Open a file write-only (create if missing; may truncate or append depending on implementation).

```rust
use std::io::Write;
let mut f = fs_ext::fsx::file::open::write_only("out.txt")?;
writeln!(f, "hello")?;
```

---

### `open::read_only`

Open a file read-only.

```rust
let mut f = fs_ext::fsx::file::open::read_only("data.bin")?;
```

---

## 🗑 Removal

### `remove`

Delete the file.

```rust
fs_ext::fsx::file::remove("obsolete.tmp")?;
```

---

### `trash`

Move to system trash/recycle bin (if supported), otherwise error.

```rust
fs_ext::fsx::file::trash("oops.txt")?;
```

---

### `trash_or_remove`

Try to trash; fall back to permanent remove if trashing fails.

```rust
fs_ext::fsx::file::trash_or_remove("old.log")?;
```

---

## 🧪 Temp

### `temp`

Create a temporary file in system temp. Auto-deletes on drop. Returns `TempFile`.

```rust
let t = fs_ext::fsx::file::temp()?;
t.as_file(); // use underlying File
```

---

### `temp_in`

Create a temp file inside a specific directory.

```rust
let t = fs_ext::fsx::file::temp_in("build/tmp")?;
```

---
