# 📦 Macros in `fs-ext`

This crate provides several macros to simplify common filesystem operations.  
They are thin wrappers around the sync API in `fs_ext::file` and `fs_ext::dir`, designed to reduce boilerplate when creating, loading, or saving files and directories.

---

## 🗂 `dir!`

Ensures a directory exists, creating it if necessary.

```rust
use fs_ext::dir;

fn main() -> std::io::Result<()> {
    let path = "data/cache";
    dir!(path)?; // creates "data/cache" if missing
    Ok(())
}
```

---

## 📄 `file!`

Two forms:

1. **Ensure a file exists**  
   Creates the file if missing, but does not overwrite contents.

   ```rust
   use fs_ext::file;

   fn main() -> std::io::Result<()> {
       let f = file!("data/config.json")?;
       Ok(())
   }
   ```

2. **Write content to a file**  
   Overwrites the file with the given content.

   ```rust
   use fs_ext::file;

   fn main() -> std::io::Result<()> {
       file!("data/message.txt", "hello world!")?;
       Ok(())
   }
   ```

---

## 📥 `load!`

Loads a serialized value from a file, inferring the format from the file extension.  
Supports formats: JSON, TOML, YAML.

```rust
use serde::Deserialize;
use fs_ext::load;

#[derive(Deserialize)]
struct Config {
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: Config = load!("data/config.json")?;
    println!("Name = {}", cfg.name);
    Ok(())
}
```

---

## 💾 `save!`

Saves a serializable value to a file, choosing the format from the extension. Supports formats: JSON, TOML, YAML.

```rust
use serde::Serialize;
use fs_ext::save;

#[derive(Serialize)]
struct Config {
    name: String,
}

fn main() -> std::io::Result<()> {
    let cfg = Config { name: "demo".into() };
    save!("data/config.json", cfg)?;
    Ok(())
}
```

---

## 📄 `tempfile!`

Creates a temporary file. Three forms:

1. **System temp directory**

   ```rust
   let tmp = tempfile!()?;
   ```

   → creates a file in the OS temp dir.

2. **Inside a specific directory**

   ```rust
   let tmp = tempfile!("data/tmp")?;
   ```

   → creates a file inside `"data/tmp"`.

3. **With initial content**

   ```rust
   use std::io::Read;

   let tmp = tempfile!("data/tmp", "hello temp!")?;
   let mut s = String::new();
   tmp.as_file().read_to_string(&mut s)?;
   assert_eq!(s, "hello temp!");
   ```

---

## 🗂 `tempdir!`

Creates a temporary directory. Two forms:

1. **System temp directory**

   ```rust
   let dir = tempdir!()?;
   println!("temp dir at {}", dir.path().display());
   ```

2. **Inside a parent directory**

   ```rust
   let dir = tempdir!("data/tmp")?;
   ```
