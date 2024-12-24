# range-proof
range proof implementation for skde

## How to Run the Code
To run this project, you will need Rust installed on your system. If Rust is not installed, you can install it from [the official Rust website](https://rust-lang.org).

### Install the risc0 toolchain
Before building the project, you must install the 'risc0' toolchain. You can install it using `rzup` as follows:
   ```bash
   curl -L https://risczero.com/install | bash
   rzup install
   ```
   
### Build the project:
   ```bash
   cargo build --release
   ```

### Run the host application:
   ```bash
   cargo run --release
   ```

## File Descriptions
- `main.rs`: Contains the main logic for range proofs for big integers using zkVM.
- `build.rs`: Script that automates the build process for the guest environment.
- `rust-toolchain.toml`: Ensures a consistent Rust version and toolchain are used across all environments to avoid compatibility issues.


## Structure
```plaintext
BIGINTRANGEPROOF
├── host
│   ├── src    
│   │   └── main.rs           
│   └── Cargo.toml            
├── methods
│   ├── guest
│   │   └── src
│   │       ├── build.rs       
│   │       └── Cargo.toml           
│   └── Cargo.toml             
├── README.md                 
└── rust-toolchain.toml       
```