# Async Implementation Guide

`fs_ext` allows easy switching between sync and async APIs:

```rust
use fs_ext::{fsx, TempDir}; // sync

use fs_ext::tokio::{fsx, TempDir} // async
```

Async is not enabled by default—you must opt into it:

```rust
cargo.toml:
fs-ext = { version = "0.1", features = ["tokio"] }
```

Currently, **Tokio** is the only async runtime supported, but the door is open for additional runtimes (e.g., `async-std`, `smol`) in the future.

### The Reality of Async File I/O

Most operating systems don't natively support truly async file operations. While modern APIs like `io_uring` are emerging, they're not yet standard across platforms. As a result async runtime crates such as **Tokio** implement file APIs (e.g., `tokio::fs`) which use `tokio::task::spawn_blocking` under the hood to wrap standard library functions, moving blocking operations to a thread pool.

### Our Async Mirror Strategy

This crate follows the same approach as `tokio::fs` for consistency and reliability:

- **Small helpers**  
  Wrap sync functions in a runtime’s _blocking task API_ (e.g., `spawn_blocking`). Both sync and async share the same core implementations and tests.
- **API consistency**: Uses `impl AsRef<Path>` parameters to mirror the sync API, even when paths get cloned internally
- **Custom async implementations**  
  For operations that benefit from **progress tracking**, **streaming**, or **early cancellation**, we write async code directly instead of wrapping sync.

### Critical Guidelines

**❌ Never do this:**

```rust
// DON'T wrap an async API in spawn_blocking
tokio::task::spawn_blocking(|| {
    fs_ext::tokio::fsx::file::create_new(path)// This is already async!
}).await
```

**Why?**

- `fs_ext::tokio::*` functions already use `spawn_blocking` internally where needed.
- Wrapping them again adds _double indirection_ and burns a thread from the blocking pool unnecessarily.

### Streaming: The Async Difference

Since Rust doesn't have async iterators, streaming operations work differently between sync and async versions.

#### Sync Streaming (Iterator-based)

```rust
use {fs_ext::fsx::file, std::io};

fn main() -> io::Result<()> {
    for line in file::stream_lines("playground/oof.txt")? {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}
```

#### Async Streaming (Custom Reader)

```rust
use {fs_ext::tokio::fsx::file, std::io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader = file::stream_bytes("playground/oof.txt", 3).await?;
    while let Some(chunk_result) = reader.next_chunk().await {
        let chunk = chunk_result?;
        println!("Read {} bytes", chunk.len());
        // Process chunk...
    }
    Ok(())
}
```
