# find

Put 

```toml
[dependencies]
find = { git = "https://github.com/cwpearson/find-rs" }
```

in your `Cargo.toml` and

```rust
extern crate find;
```

in your Rust file.

## Example

```rust
    const SEARCH_LINUX: &[&str] = &["/usr/local/cuda/lib*", "/usr/local/cuda*/lib*"];

    // ...

    let cudart_path = match Find::new("libcudart.so.*")
        .search_env("LIBCUDA_PATH")
        .search_globs(SEARCH_LINUX)
        .execute()
    {
        Ok(path) => path,
        Err(message) => panic!(message),
    };

    let lib_path = cudart_path.parent().unwrap();
    eprintln!("Found cudart: {:?}", cudart_path);

    // Discover the version of the CUDA libraries
    // Set the corresponding feature flags
    if let Some(version) = find::parse_version(&cudart_path) {
        if version.len() > 0 {
            println!("libcudart.so version {}", version[0]);
        }
    }

```