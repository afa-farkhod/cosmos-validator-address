use bech32::{self, ToBase32, Variant};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};
use cosmrs::crypto::PublicKey;
use cosmrs::tendermint;

fn main() {
    // Sample base64 pub key
    let base64_pubkey = "bDO9bUrbyg0f1pTpmjjZU5cgsweCWdwL6HUVnsKJi7k=";

    // Decode base64 public key
    let decoded_bytes = general_purpose::STANDARD.decode(base64_pubkey)
        .expect("Failed to decode base64");

    // Convert raw bytes into Tendermint PublicKey (Ed25519)
    let tendermint_pubkey = tendermint::PublicKey::from_raw_ed25519(&decoded_bytes)
        .expect("Invalid public key");

    // Convert Tendermint PublicKey to CosmRS PublicKey
    let pubkey = PublicKey::from(tendermint_pubkey);

    // Compute the address (SHA-256 hash of the pubkey)
    let pubkey_bytes = pubkey.to_bytes();
    let hash = Sha256::digest(pubkey_bytes);
    let truncated_address = &hash[..20]; // Take first 20 bytes

    // Encode in Bech32 with the prefix "cosmosvalcons"
    let bech32_address = bech32::encode("cosmosvalcons", truncated_address.to_base32(), Variant::Bech32)
        .expect("Bech32 encoding failed");

    println!("Cosmos Validator Address: {}", bech32_address);
}
