# 📁 `fs_ext::dir` — Directory Utilities

Utilities for validating, creating, querying, and manipulating directories.  
Designed to give clear errors (with path context) and sensible, safe defaults.

---

## 📜 Index

- **Checks**
  - [`assert_exists`](#assert_exists)
  - [`assert_not_exists`](#assert_not_exists)
  - [`exists`](#exists)
  - [`is_empty`](#is_empty)
  - [`size`](#size)
- **Creation**
  - [`create_new`](#create_new)
  - [`ensure`](#ensure)
- **Query (via [`DirQuery`](./dirquery.md))**
  - [`entries`](#entries)
  - [`files`](#files)
  - [`subdirs`](#subdirs)
- **Temp**
  - [`temp`](#temp)
  - [`temp_in`](#temp_in)
- **Utils**
  - [`clear`](#clear)
  - [`copy_contents`](#copy_contents)
  - [`copy`](#copy)

---

## 🔎 Checks

### `assert_exists`

Ensures the path exists _and_ is a directory.

```rust
use fs_ext::dir;

dir::assert_exists("data")?;
```

- **Ok**: Path exists and is a directory.
- **Err**: `NotFound` if missing; `InvalidInput` if not a directory; other I/O errors propagated.

---

### `assert_not_exists`

Ensures the path **does not** exist (neither file nor directory).

```rust
use fs_ext::dir;

dir::assert_not_exists("data/new-project")?;
```

- **Ok**: Path is absent.
- **Err**: `AlreadyExists` if something exists there; other I/O errors propagated.

---

### `exists`

Returns whether a path exists **and is a directory**.

```rust
use fs_ext::dir;

if dir::exists("cache")? {
    // ...
}
```

- **Ok(true/false)**: Only true if the path exists and is a **dir**.
- **Err**: Non-NotFound metadata errors are propagated.

---

### `is_empty`

Checks whether a directory contains **no entries**.

```rust
use fs_ext::dir;

if dir::is_empty("staging")? {
    // safe to reuse
}
```

- **Ok(true)**: No entries at all.
- **Ok(false)**: At least one entry is present.
- **Err**: Missing path, not a directory, or read errors → error.

---

### `size`

Computes total byte size of **all regular files** under the directory (recursive).

```rust
use fs_ext::dir;

let bytes = dir::size("assets")?;
println!("assets/ total size: {bytes} bytes");
```

---

## 🧱 Creation

### `create_new`

Creates a **new** directory. Fails if it already exists.

```rust
use fs_ext::dir;

dir::create_new("output")?; // like `mkdir` without -p
```

---

### `ensure`

Creates a directory **and all parents** if missing (idempotent).

```rust
use fs_ext::dir;

dir::ensure("var/tmp/cache")?; // like `mkdir -p`
```

---

## 🔍 Query (via `DirQuery`)

These helpers build a [`DirQuery`](./dirquery.md) with common presets.  
Use them and then call `.collect()`, `.count()`, or `.exists()`.

### `entries`

All entries (files **and** directories), recursive by default.

```rust
use fs_ext::dir;

let all = dir::entries("src").collect()?;
```

### `files`

Only files, recursive by default.

```rust
use fs_ext::dir;

let rs_files = dir::files("src")
    .filter_extension("rs")
    .collect()?;
```

### `subdirs`

Only directories, recursive by default.

```rust
use fs_ext::fsx::dir;

let modules = dir::subdirs("src").depth(1).collect()?;
```

> Each returns a `DirQuery` you can further configure:
>
> - `.recursive(bool)`
> - `.depth(usize)`
> - `.limit(usize)`
> - `.filter_extension(...)`, `.exclude_extension(...)`
> - `.include_files(bool)`, `.include_dirs(bool)`

---

## 🧪 Temp

### `temp`

Create a temporary directory in the system temp location. Auto-deletes on drop.

```rust
use fs_ext::fsx::dir;

let t = dir::temp()?;
let p = t.path();
// do work in `p`; removed when `t` drops
```

- Returns a `TempDir` with `path()`, `keep()`, `close()`.

### `temp_in`

Create a temporary directory under a **specific parent**.

```rust
use fs_ext::fsx::dir;

let t = dir::temp_in("build/tmp")?;
```

---

## 🧰 Utils

### `clear`

Remove **all contents** of a directory, but keep the directory itself.

```rust
use fs_ext::fsx::dir;

dir::clear("target/tmp")?; // empties directory
```

---

### `copy_contents`

Copy **all entries** from a source directory into a destination directory (recursive).

```rust
use fs_ext::fsx::dir;

dir::copy_contents("templates/base", "project")?;
```

---

### `copy`

Copy the `src` directory into `dst`.

```rust
use fs_ext::fsx::dir;

dir::copy("svgs", "build/assets")?;
```
