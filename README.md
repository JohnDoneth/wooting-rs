# Wooting-rs

[⚠ Heavily WIP ⚠] Wooting Keyboard RGB & Analog crate for Rust

Feel free to issue a pull request or raise an issue!

I currently don't have a Wooting keyboard to make sure the code is behaving correctly. Once, I do (by December?), a lot more testing and development will occur.

If you do already have a Wooting keyboard your help would be much appreciated!

## Build & Run

- Bindgen requires clang. You can find the requirements here https://rust-lang-nursery.github.io/rust-bindgen/requirements.html

```shell
git submodule update --init --recursive

cargo run --examples hello
```

## Issues

- The way the hidapi library is currently built and linked currently only works for Windows. This should be easy to fix by modifying the build scripts.
- General API implementation, some of the API surface is currently not covered.
- Lack of testing