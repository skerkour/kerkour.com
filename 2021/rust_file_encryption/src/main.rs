use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};
use rand::RngCore;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    println!("Encrypting 100.bin to 100.encrypted");
    encrypt_small_file("100.bin", "100.encrypted")?;

    println!("Encrypting 2000.bin to 2000.encrypted");
    encrypt_large_file("1000.bin", "1000.encrypted")?;

    Ok(())
}

fn encrypt_small_file(filepath: &str, dist: &str) -> Result<(), anyhow::Error> {
    let mut rand_generator = rand::rngs::OsRng {};
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];

    rand_generator.fill_bytes(&mut key);
    rand_generator.fill_bytes(&mut nonce);
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());

    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(&nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
}

fn encrypt_large_file(filepath: &str, dist: &str) -> Result<(), anyhow::Error> {
    Ok(())
}
