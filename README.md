# rust_fzf
Rust library to allow for selecting an item using the `fzf` CLI tool.

## Install
```
cargo add rust_fzf
```

## Usage
```rust
let selected: Result<Vec<String>, String> = select(
    vec!["hello".to_string(), "rust_fzf!".to_string()],
    vec!["--layout=reverse".to_string()],
);
```
