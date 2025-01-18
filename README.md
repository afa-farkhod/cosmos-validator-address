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

# expected output:
Cosmos Validator Address: cosmosvalcons1lth4cv5tkn2fc5yuy4xx6rj7wvzrpxkkqg3my5
```

- Example Ed25519 public key input:
```
{
    "consensus_pubkey": {
      "@type": "/cosmos.crypto.ed25519.PubKey",
      "key": "bDO9bUrbyg0f1pTpmjjZU5cgsweCWdwL6HUVnsKJi7k="
    }
}
```

📜 Explanation
1. Decodes the base64 string to raw bytes.
2. Parses it as a Tendermint public key.
3. Hashes the public key with SHA-256 and truncates it to 20 bytes.
4. Encodes it using Bech32 with the prefix cosmosvalcons.

🏗 Contributing

- Pull requests are welcome! Please open an issue if you find bugs or need improvements.


