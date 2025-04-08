# range-proof
range proof implementation for skde

## How to Run the Code
To run this project, you will need Rust installed on your system. If Rust is not installed, you can install it from [the official Rust website](https://rust-lang.org).

### Install prerequisites
On Mac:
```
brew install cmake
brew install ninja
xcode-select --install
```

### Install the risc0 toolchain
Before building the project, you must install the 'risc0' toolchain. You can install it using `rzup` as follows:
```bash
curl -L https://risczero.com/install | bash
rzup install
```
For detailed installation instructions, refer to the official documentation: [RISC Zero zkVM Installation Guide](https://dev.risczero.com/api/zkvm/install)

### Run the project:
```bash
cargo run --release
```

### Run the host application:
```bash
cargo run --bin host --release
```

### Troubleshooting

If you encounter build errors related to the `risc0` toolchain (e.g., when using older versions like `v1.0.5`), please ensure the following:

- Use the latest `risc0` version (`v2.0.1` or above)
- Install Rust `1.86.0` via `rustup`
- Update your `rzup`, `cargo-risczero`, and `r0vm` tools:
  ```bash
  rzup update
  rzup install cargo-risczero 2.0.1
  rzup install r0vm 2.0.1
  rzup default cargo-risczero 2.0.1
  rzup default r0vm 2.0.1
  ```
- Clean previous builds:
  ```bash
  cargo clean
  ```

Then try running the host again:
```bash
cargo run --bin host --release
```
