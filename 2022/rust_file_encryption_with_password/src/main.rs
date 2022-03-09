use anyhow::{anyhow, Context};
use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use std::{
    env,
    fs::File,
    io::{Read, Write},
};
use zeroize::Zeroize;

const BUFFER_LEN: usize = 500;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: ./encryptor <file>"));
    }

    let file = args[1].clone();
    let mut password = rpassword::prompt_password_stdout("password:")?;

    if file.ends_with(".encrypted") {
        let dest = file.strip_suffix(".encrypted").unwrap().to_string() + ".decrypted";
        decrypt_file(&file, &dest, &password)?;
    } else {
        let dest = file.clone() + ".encrypted";
        encrypt_file(&file, &dest, &password)?;
    }

    password.zeroize();

    Ok(())
}

fn argon2_config() -> argon2::Config<'static> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    }
}

fn encrypt_file(
    source_file_path: &str,
    dest_file_path: &str,
    password: &str,
) -> Result<(), anyhow::Error> {
    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    let mut source_file = File::open(source_file_path)?;
    let mut dest_file = File::create(dest_file_path)?;

    dest_file.write_all(&salt)?;
    dest_file.write_all(&nonce)?;

    let mut buffer = vec![0; BUFFER_LEN + 16];
    let mut filled = 0;

    loop {
        // We leave space for the tag
        let read_count = source_file.read(&mut buffer[filled..BUFFER_LEN])?;
        filled += read_count;

        if filled == BUFFER_LEN {
            buffer.truncate(BUFFER_LEN);
            stream_encryptor
                .encrypt_next_in_place(&[], &mut buffer)
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dest_file.write_all(&buffer)?;
            filled = 0;
        } else if read_count == 0 {
            buffer.truncate(filled);
            stream_encryptor
                .encrypt_last_in_place(&[], &mut buffer)
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dest_file.write_all(&buffer)?;
            break;
        }
    }

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(())
}

fn decrypt_file(
    encrypted_file_path: &str,
    dest: &str,
    password: &str,
) -> Result<(), anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];

    let mut encrypted_file = File::open(encrypted_file_path)?;
    let mut dest_file = File::create(dest)?;

    encrypted_file
        .read_exact(&mut salt)
        .context("Error reading salt.")?;

    encrypted_file
        .read_exact(&mut nonce)
        .context("Error reading nonce.")?;

    let argon2_config = argon2_config();

    let mut key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());
    let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

    // ⚠ 16 bytes for the Tag appended by any Poly1305 variant
    let mut buffer = vec![0u8; BUFFER_LEN + 16];
    let mut filled = 0;

    loop {
        // here we fill all the way to BUFFER_LEN + 16, so we can omit the range end
        let read_count = encrypted_file.read(&mut buffer[filled..])?;
        filled += read_count;

        if filled == BUFFER_LEN + 16 {
            stream_decryptor
                .decrypt_next_in_place(&[], &mut buffer)
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dest_file.write_all(&buffer)?;
            filled = 0;
            buffer.extend([0; 16]);
        } else if read_count == 0 {
            buffer.truncate(filled);
            stream_decryptor
                .decrypt_last_in_place(&[], &mut buffer)
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dest_file.write_all(&buffer)?;
            break;
        }
    }

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(())
}

#[test]
fn roundtrip() {
    let source_file_path = "file.bin";
    let dest_file_path = "file.bin.encrypted";
    let password = "a very secure password!";
    let decrypted_file_path = "file.bin.decrypted";
    encrypt_file(source_file_path, dest_file_path, password).unwrap();
    decrypt_file(dest_file_path, decrypted_file_path, password).unwrap();
    assert_eq!(
        std::fs::read(source_file_path).unwrap(),
        std::fs::read(decrypted_file_path).unwrap()
    );
}
