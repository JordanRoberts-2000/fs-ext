# 📂 `DirQuery`

`DirQuery` is a builder-style API for scanning directory contents with fine-grained control over recursion, depth, inclusion, and file extension filters.

---

## 🔨 Construction

```rust
use fs_ext::DirQuery;

let q = DirQuery::new("some/dir");
```

- Defaults:
  - includes **files** and **dirs**
  - is **recursive**
  - no depth or limit
  - no extension filters

### Convenience constructors (`fs_ext::dir`)

You can also create common queries directly via helpers in `fs_ext::dir`:

```rust
use fs_ext::dir;

// Equivalent to DirQuery::new(path)
let q1 = dir::entries("some/dir");

// Only files
let q2 = dir::files("some/dir");

// Only subdirectories
let q3 = dir::subdirs("some/dir");
```

These return a `DirQuery` already configured for typical use cases.

---

## ⚙️ Configuration Methods

### Include/exclude kinds

```rust
fn include_files(self, bool) -> Self
fn include_dirs(self, bool) -> Self
```

Enable/disable whether files or directories appear in results.

---

### Traversal options

```rust
fn recursive(self, bool) -> Self
fn limit(self, n: usize) -> Self
fn depth(self, n: usize) -> Self
```

- `recursive(false)` → restrict to the top-level directory only.
- `limit(n)` → cap the number of returned entries.
- `depth(n)` → maximum recursion depth (`1` = only immediate children).

---

### Extension filters

```rust
fn filter_extension(self, ext: impl AsRef<str>) -> Self
fn filter_extensions<I, S>(self, exts: I) -> Self
fn exclude_extension(self, ext: impl AsRef<str>) -> Self
fn exclude_extensions<I, S>(self, exts: I) -> Self
```

- **filter\_\*** → whitelist certain extensions.
- **exclude\_\*** → blacklist certain extensions.
- Normalizes extensions so `"rs"`, `".rs"` → both treated the same.
- If filters are combined, exclusion wins.

---

## 📊 Execution Methods

```rust
fn collect(self) -> io::Result<Vec<PathBuf>>
fn count(self) -> io::Result<usize>
fn exists(self) -> io::Result<bool>
```

- `collect` → materialize into a vector of `PathBuf`.
- `count` → number of matching entries.
- `exists` → returns true if at least one match exists.

---

## 💡 Example Usages

### Collect all Rust source files, recursive

```rust
use fs_ext::DirQuery;

let rs_files = DirQuery::new("src")
    .filter_extension("rs")
    .collect()?;
```

---

### Limit to 10 entries

```rust
let first_ten = fs_ext::dir::files("logs")
    .limit(10)
    .collect()?;
```

---

### Check if any `.jpg` exists at top-level

```rust
let has_jpg = fs_ext::dir::files("images")
    .recursive(false)
    .filter_extension("jpg")
    .exists()?;
```

---

### Count nested directories up to depth 2

```rust
let dir_count = fs_ext::dir::subdirs("workspace")
    .depth(2)
    .count()?;
```
