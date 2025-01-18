# cosmos-validator-address
Cosmos Validator Address Conversion using `cosmrs`

This repository provides a Rust implementation for converting a `base64-encoded Ed25519 public key` into a `Bech32-encoded` `cosmosvalcons` validator address using the [`cosmrs`](https://docs.rs/cosmrs/latest/cosmrs/) library.

## 📌 Features
- Decodes a **base64-encoded** public key.
- Converts it into a **Tendermint Ed25519 public key**.
- Computes the **SHA-256 hash** and extracts the first 20 bytes.
- Encodes the result in **Bech32** format with the `cosmosvalcons` prefix.

---

## 🚀 Getting Started

### 1️⃣ **Install Rust**
If you haven't installed Rust yet, install it using [rustup](https://rustup.rs/):

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

#Verify installation:
rustc --version
cargo --version
```

2️⃣ Clone the Repository
```
git clone https://github.com/afa-farkhod/cosmos-validator-address.git
cd cosmos-validator-address
```

3️⃣ Add Dependencies
Make sure your `Cargo.toml` contains the following dependencies:
```
[package]
name = "cosmos-validator-address"
version = "0.1.0"
edition = "2021"

[dependencies]
cosmrs = "0.15"
bech32 = "0.9"
sha2 = "0.10"
base64 = "0.21"
```

4️⃣ Usage
Replace the base64 public key in `src/main.rs` with your own, then run:
```
cargo run
```
