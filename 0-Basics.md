# Rust Fundamentals
**Explore:** [Home](/README.md) [Testing](/0-Testing.md)

## Cheatsheet —
- std::fmt – [[[fill]align][sign]['#']['0'][width]['.' precision]type](https://doc.rust-lang.org/std/fmt/index.html#syntax)
    - format!() ∙∙∙∙∙∙∙ print!() ∙∙∙∙∙∙∙ eprint!()
    - `{}` fmt::Display ∙∙∙∙∙∙∙ `{:?}` `{:#?}` fmt::Debug
    - `{:b}` binary ∙∙∙∙∙∙∙ `{:o}` octal ∙∙∙∙∙∙∙ `{:x}` hex

## Cargo Cheatsheet —
- `cargo new <project_name> --lib`
- `cargo [check | run | build --release]>`

### Cargo.toml —
```toml
[workspace]
resolver = "2"
members = ["projects/*"]
# ...............................
[dependencies]
utils = { path = "../utils" }
# ...............................
[lints.rust]
# dead_code = "allow"
# unused_variables = "allow"
# unused_imports = "allow"
```
- [Cargo Manifest | The Cargo Book](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Cargo Workspaces | The Rust Book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

## Env Setup —
- Windows Def. Install Path: `C:\Users\<user> → ~\.rustup & ~\.cargo`
- [Install | Rust-lang.org](https://www.rust-lang.org/tools/install)
- [Downloads | VS Code](https://code.visualstudio.com/download)
