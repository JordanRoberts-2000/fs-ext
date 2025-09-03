# 🔧 Traits in `fs-ext`

- **`IoResultExt`**: enrich `io::Result` errors with human-friendly path context.
- **`PathExt`**: strict `Path` checks + assertions (`is_*_strict`, `assert_*`, `kind`).
- **`Format`**: bring-your-own (de)serializer; get `load`/`save` with atomic writes.

These traits aim to keep your call sites concise, your errors meaningful, and your file IO safer.
