
# Wooting-rs

[Heavily WIP] Wooting Keyboard RGB & Analog crate for Rust

Feel free to issue a pull request or raise an issue!

I currently don't have a Wooting keyboard to make sure the code is behaving correctly.

## Build & Run

```shell
git submodule update --init --recursive

cargo run --examples hello
```

## Issues

- The way the hidapi library is currently built and linked currently only works for Windows. This should be easy to fix by modifying the build scripts.
- General API implementation