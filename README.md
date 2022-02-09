# rust-stb-image

Rust bindings to the awesome [stb_image](https://github.com/nothings/stb) library.

[Documentation](https://docs.rs/stb_image/)

Currently using stb_image v2.22.

## Features

The default is to use libc to avoid compatibility problems with old projects using this crate.

You can use `std::os::raw` types instead by specifying `--no-default-features` in your `Cargo.toml`