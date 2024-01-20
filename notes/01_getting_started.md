## Getting Started With Rest

### Installing Rust on Mac

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
# Reload or relaunch your terminal for $PATH to be updated or source ~/.profile

rustup --version
rustup —help

rustup update
```
When you install rust you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:
```bash
cargo --version
```

### Creating a new project
```bash
cargo new hello-rust
cd hello-rust

tree
# .
# ├── Cargo.toml
# └── src
#     └── main.rs
 ```

 ### Code Editor - VS Code

* Install rust extension - `rust-analyzer`
* Install Toml extension -`Even Better TOML`

Follow detailed instructions [here]( https://code.visualstudio.com/docs/languages/rust)

### Running Rust Program

```
cargo run
cargo run -q

# Or, click `run` from VS Code editor (requires Rust Plugin)
``` 