# Known Limitations

This document outlines current limitations of `fs_ext`, along with some context on why they exist and possible future directions.

---

## Async TempFile Copy

The async tokio variant of `TempFile::copy_from` currently **requires a Tokio multi-threaded runtime**.

- Internally, this functions calls `block_on`, which requires a multi-threaded runtime. This is because it struggles to safely move a `&mut` reference into the blocking closure.
- A potential fix could be to refactor `TempFile` to wrap its internals in an `Arc` with a synchronization primitive (e.g., `Mutex` or `RwLock`), allowing safe shared access across threads.

This limitation also affects some async atomic functions, which depend on `TempFile::copy_from`.

---

## Symlink Handling

At present, **symbolic links are ignored** by most operations. This is intentional for now: the crate is primarily used as a building block for personal developer tooling and other crates, where symlinks haven't been relevant.

That said, acknowledging and handling symlinks more explicitly is a **future goal**. Contributions, use cases, or suggestions here are welcome.
