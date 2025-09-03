# 📦 fs-ext

`fs-ext` extends the standard library’s filesystem API with safe, atomic, and ergonomic utilities. It reduces boilerplate and gives you practical tools for everyday file and directory operations.

---

## ✨ Features

- 🧩 **Macros**: quick shorthands (`file!`, `dir!`, `load!`, `save!`, `temp!`)
- 🛠 **Traits**: extend `io::Result` and `Path` with context and strict checks
- 📁 **Dir utilities**: ensure, assert, clear, copy, temp dirs, and querying with `DirQuery`
- 📄 **File utilities**: create, touch, append, read, stream, save/load typed models
- 💾 **Typed saving/loading**: JSON/TOML/YAML with extension inference or explicit format
- ⚡ **Async support**: optional `tokio` feature providing async mirrors of all APIs
- 🧨 **Atomic operations**: create/update/overwrite files safely
- 🧪 **Temp files/dirs**: RAII-managed, auto-cleanup, with `keep()`/`persist()` options
- 🔎 **Context-rich errors**: never guess which path failed again

---

## 🚀 Quickstart

Add to your `Cargo.toml`:

```toml
[dependencies]
fs-ext = "0.1"
```

---

## ⚡ Async Support

`fs-ext` provides both **sync** and **async** APIs with the same interface:

```rust
use fs_ext::fsx;           // sync
use fs_ext::tokio::fsx;    // async
```

Enable async with the `tokio` feature:

```toml
fs-ext = { version = "0.1", features = ["tokio"] }
```

- Currently supports **Tokio** as the async runtime.
- Async APIs mirror sync ones.
- Streaming APIs differ slightly: sync uses iterators, async uses custom readers.

This makes it easy to switch between blocking and async contexts without changing your code structure. (see [async-guide](./docs/async-guide.md) for details)

## 🧰 Examples

### Ensure directories & files

```rust
use fs_ext::fsx::{dir, file};

dir::ensure("data/cache")?;
file::ensure("data/cache/index.json")?;
```

---

### Write and read typed data

```rust
use fs_ext::fsx::file;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    user: String,
    retries: u32,
}

let cfg = Config { user: "demo".into(), retries: 3 };

// Save (inferred from extension)
file::save_auto("config.json", &cfg)?;

// Load
let loaded: Config = file::load_auto("config.json")?;
```

---

### Atomic update

```rust
use std::io::Write;
use fs_ext::fsx::file;

file::atomic::update("counter.txt", |current, f| {
    let n: u64 = current.trim().parse().unwrap_or(0);
    write!(f, "{}", n + 1)?;
    Ok(())
})?;
```

---

### Temp files and dirs

```rust
use fs_ext::fsx::file;

let mut tmp = file::temp()?;              // auto-deletes on drop
tmp.as_file_mut().write_all(b"scratch")?;

let (f, path) = tmp.keep()?;              // keep permanently
```

---

### Directory queries

```rust
use fs_ext::DirQuery;

let rs_files = DirQuery::new("src")
    .filter_extension("rs")
    .collect()?;

// Or use convenience
let only_dirs = fs_ext::fsx::dir::subdirs("src").collect()?;
```

---

### Macros

```rust
file!("hello.txt", "hello world")?;
let cfg = load!("config.json")?;
save!("config.json", cfg)?;
let tmp = tempfile!(".", "scratch")?;
```

---

## 📚 Documentation

- [API Reference](./docs/api-reference.md)
- [File utilities](./docs/file.md)
- [Dir utilities](./docs/dir.md)
- [DirQuery builder](./docs/dirquery.md)
- [Traits](./docs/traits.md)
- [Macros](./docs/macros.md)
- [Async-guide](./docs/async-guide.md)
- [Limitations](./docs/limitations.md)

---

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).

---

## 📜 License

Dual-licensed under either:

- MIT License ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
