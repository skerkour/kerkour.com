use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};
use rand::RngCore;
use std::{
    fs::{self, File},
    io::{Read, Write},
};

fn main() -> Result<(), anyhow::Error> {
    let mut rand_generator = rand::rngs::OsRng {};

    let mut small_file_key = [0u8; 32];
    let mut small_file_nonce = [0u8; 24];
    rand_generator.fill_bytes(&mut small_file_key);
    rand_generator.fill_bytes(&mut small_file_nonce);

    println!("Encrypting 100.bin to 100.encrypted");
    encrypt_small_file(
        "100.bin",
        "100.encrypted",
        &small_file_key,
        &small_file_nonce,
    )?;

    println!("Decrypting 100.encrypted to 100.decrypted");
    decrypt_small_file(
        "100.encrypted",
        "100.decrypted",
        &small_file_key,
        &small_file_nonce,
    )?;

    println!("Encrypting 2048.bin to 2048.encrypted");
    encrypt_large_file("2048.bin", "2048.encrypted")?;

    println!("Decrypting 2048.encrypted to 2048.decrypted");
    decrypt_large_file("2048.encrypted", "2048.decrypted")?;

    Ok(())
}

fn encrypt_small_file(
    filepath: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(filepath)?;

    let encrypted_file = cipher
        .encrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Encrypting small file: {}", err))?;

    fs::write(&dist, encrypted_file)?;

    Ok(())
}

fn decrypt_small_file(
    encrypted_file_path: &str,
    dist: &str,
    key: &[u8; 32],
    nonce: &[u8; 24],
) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(encrypted_file_path)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

    fs::write(&dist, decrypted_file)?;

    Ok(())
}

fn encrypt_large_file(filepath: &str, dist: &str) -> Result<(), anyhow::Error> {
    let mut rand_generator = rand::rngs::OsRng {};
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];

    rand_generator.fill_bytes(&mut key);
    rand_generator.fill_bytes(&mut nonce);
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());

    const BUFFER_LEN: usize = 1000;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut file = File::open(filepath)?;
    let mut dist_file = File::create(dist)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        dist_file.write(&buffer[..read_count])?;

        if read_count != BUFFER_LEN {
            break;
        }
    }

    Ok(())
}

fn decrypt_large_file(encrypted_file_path: &str, dist: &str) -> Result<(), anyhow::Error> {
    let mut rand_generator = rand::rngs::OsRng {};
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 24];

    rand_generator.fill_bytes(&mut key);
    rand_generator.fill_bytes(&mut nonce);
    let cipher = XChaCha20Poly1305::new(key.as_ref().into());

    const BUFFER_LEN: usize = 1000;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut file = File::open(encrypted_file_path)?;
    let mut dist_file = File::create(dist)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        dist_file.write(&buffer[..read_count])?;

        if read_count != BUFFER_LEN {
            break;
        }
    }

    Ok(())
}
