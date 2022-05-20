# unveil-rs

[![Crate](https://img.shields.io/crates/v/unveil.svg)](https://crates.io/crates/unveil)
[![Documentation](https://docs.rs/unveil/badge.svg)](https://docs.rs/unveil)

Rust binding for OpenBSD's [unveil(2)](https://man.openbsd.org/unveil.2).

## Requirements

- OpenBSD 6.4 or later

## Usage

```rust
extern crate unveil;

use std::fs::File;
use std::io::prelude::*;
use unveil::unveil;

fn main() {
    let path = "public.txt";
    let contents = b"Hello world!";
    File::create(path).unwrap().write_all(contents).unwrap();

    // Restrict filesystem view by only allowing read operations on the specified path
    unveil(path, "r")
    .or_else(unveil::Error::ignore_platform)
    .unwrap();

    // Reading from unveiled paths will succeed
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    assert_eq!(contents, &buffer[..]);

    // Reading from paths which have not been unveiled will fail
    assert!(File::open("/etc/passwd").is_err());

    // Disable further calls to unveil
    unveil("", "")
    .or_else(unveil::Error::ignore_platform)
    .unwrap();

    // All calls to unveil will now fail
    assert!(unveil(path, "rw").is_err());
}
```

## Related projects

- [pledge-rs](https://crates.io/crates/pledge) - Rust binding for OpenBSD's pledge(2).
