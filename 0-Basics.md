# Rust Fundamentals
**Explore:** [Home](/README.md) [Testing](/0-Testing.md)

## Toolbox —
- <span title="automatically imported into every Rust program">Rust Prelude</span>
    - `std::fmt` – [[[fill]align][sign]['#']['0'][width]['.' precision]type](https://doc.rust-lang.org/std/fmt/index.html#syntax)
        - format!() ∙∙∙∙∙∙∙ print!() ∙∙∙∙∙∙∙ eprint!() ∙∙∙∙∙∙∙ write!()
        - `{}` fmt::Display ∙∙∙∙∙∙∙ `{:?}` `{:#?}` fmt::Debug
        - `{:b}` binary ∙∙∙∙∙∙∙ `{:o}` octal ∙∙∙∙∙∙∙ `{:x}` hex
        - `fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {...}`
    - Result<Ok(T), Err(_)>.expect()
    - Option<>

## Cargo Cheatsheet —
- `cargo new <project_name> --lib`
- `cargo doc --open`
- `cargo [check | run | build --release]`
- <span title="For patching dependencies; minor/major versions must be manually updated">`cargo update`</span>

### Cargo.toml —
```toml
[workspace]
resolver = "3"
members = ["projects/*"]
# ...............................
[dependencies]      # Uses SemVer; 0.4.2 shorthand for ^0.4.2, i.e. [0.4.2, 0.5.0)
utils = { path = "../utils" }
# ...............................
[lints.rust]
# dead_code = "allow"
# unused_variables = "allow"
# unused_imports = "allow"
```
- [Cargo Manifest | The Cargo Book](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Cargo Workspaces | The Rust Book](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Package Registry | Crates.io](https://crates.io/)

## Env Setup —
- Windows Def. Install Path: `C:\Users\<user> → ~\.rustup & ~\.cargo`
- Custom Cargo commands → `~\.cargo\config.toml`
    - ```toml
      [alias]
      # Custom alias 'cargo test-verbose' to run tests with non-captured output
      test-verbose = "test -- --nocapture"
      ```
- [Install | Rust-lang.org](https://www.rust-lang.org/tools/install)
- [Downloads | VS Code](https://code.visualstudio.com/download)
- In VSCode workspace, add `.vscode/settings.json` and list Cargo.toml's (for rust-analyzer extension)
    - ```json
      {
          "rust-analyzer.linkedProjects": [
              ".\\hello-rust\\Cargo.toml",
              ".\\ratatui\\counter-app\\Cargo.toml",
              ".\\ratatui\\json-editor\\Cargo.toml"
          ]
      }
      ```