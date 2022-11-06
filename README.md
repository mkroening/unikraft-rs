# unikraft-rs

This crate builds and links against [Unikraft].

[Unikraft]: https://github.com/unikraft/unikraft

## Requirements

* [KraftKit](https://github.com/unikraft/kraftkit)
* [Rust](https://www.rust-lang.org/tools/install)
    * Either [Rust for Unikraft](https://github.com/unikraft/rust) (`x86_64-unikraft`)
    * Or Rust nightly (`x86_64-unknown-linux-gnu`, `no-std` only)

## Supported Unikraft platforms

* `kvm`
* `linuxu`

## Usage

You can compile the [examples] like this:

```console
cargo build \
    --example <EXAMPLE> \
    --features <PLATFORM> \
    --target <TRIPLE>
```

[examples]: examples

You can only build `no-std` applications using the `x86_64-unknown-linux-gnu` target.
This target requires additional `RUSTFLAGS`, which are defined in [`.cargo/config.toml`].

[`.cargo/config.toml`]: .cargo/config.toml
