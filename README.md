# link-line helper to build correct cdylibs

[![LICENSE](https://img.shields.io/badge/license-BSD2-blue.svg)](LICENSE)

## Supported targets

- Linux and Android
- macOS and iOS
- Windows (gnu)

## Usage

### build.rs

Add the crate to your [build-dependencies](https://doc.rust-lang.org/cargo/reference/manifest.html#dependency-sections), in your `build.rs`, call `metabuild()`.

``` toml
[build-dependencies]
cdylib-link-lines = "0.1"
```

``` rust
fn main() {
    cdylib_link_lines::metabuild();
}
```
### metabuild

If you are using the `metabuild` [unstable feature](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#metabuild)

``` toml
cargo-features = ["metabuild"]

[package]
name = "mypackage"
...
metabuild = ["cdylib-link-lines"]

[build-dependencies]
cdylib-link-lines = "0.1"
```

## Credits

Helper spun off [crav1e](https://github.com/lu-zero/crav1e), contains code written by Luca Barbato and Derek Buitenhuis.
Synchronized with the [cargo-c](https://github.com/lu-zero/cargo-c) 0.9 logic thanks to Ivan Enderlin.
