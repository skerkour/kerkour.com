use anyhow::{anyhow, Context};
use chacha20poly1305::{
    aead::{stream, NewAead},
    XChaCha20Poly1305,
};
use rand::{rngs::OsRng, RngCore};
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::PathBuf,
};
use zeroize::Zeroize;

const MSG_LEN: usize = 500;
/// the size of `chacha20poly1305::Tag`;
const TAG_LEN: usize = 16;

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args();
    let path = match (args.next(), args.next(), args.next()) {
        (Some(_), Some(file_path), None) => file_path,
        _ => return Err(anyhow!("Usage: ./encryptor <file>")),
    };
    let mut path: PathBuf = path.try_into()?;
    let file = BufReader::new(File::open(&path)?);
    let ext = path.extension();
    let decrypt = ext.map_or(false, |ext| ext == "encrypted");
    if decrypt {
        path.set_extension("decrypted");
    } else if let Some(ext) = ext {
        let mut ext = ext.to_owned();
        ext.push(".encrypted");
        path.set_extension(ext);
    } else {
        path.set_extension("encrypted");
    }
    let out_file = BufWriter::new(File::create(path)?);

    let mut password = rpassword::prompt_password_stdout("password:")?;

    if decrypt {
        decrypt_file(file, out_file, &password)?;
    } else {
        encrypt_file(file, out_file, &password)?;
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
    mut source_file: impl Read,
    mut dest_file: impl Write,
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

    dest_file.write_all(&salt)?;
    dest_file.write_all(&nonce)?;

    let mut buffer = vec![0; MSG_LEN + TAG_LEN];
    let mut filled = 0;

    loop {
        // We leave space for the tag
        let read_count = source_file.read(&mut buffer[filled..MSG_LEN])?;
        filled += read_count;

        if filled == MSG_LEN {
            buffer.truncate(MSG_LEN);
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
    mut encrypted_file: impl Read,
    mut dest_file: impl Write,
    password: &str,
) -> Result<(), anyhow::Error> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 19];

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

    // ⚠ TAG_LEN bytes for the Tag appended by any Poly1305 variant
    let mut buffer = vec![0u8; MSG_LEN + TAG_LEN];
    let mut filled = 0;

    loop {
        // here we fill all the way to MSG_LEN + TAG_LEN, so we can omit the range end
        let read_count = encrypted_file.read(&mut buffer[filled..])?;
        filled += read_count;

        if filled == MSG_LEN + TAG_LEN {
            stream_decryptor
                .decrypt_next_in_place(&[], &mut buffer)
                .map_err(|err| anyhow!("Decrypting large file: {}", err))?;
            dest_file.write_all(&buffer)?;
            filled = 0;
            buffer.extend([0; TAG_LEN]);
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
fn roundtrip_file() {
    let source_file_path = "file.bin";
    let dest_file_path = "file.bin.encrypted";
    let password = "a very secure password!";
    let decrypted_file_path = "file.bin.decrypted";
    encrypt_file(
        BufReader::new(File::open(source_file_path).unwrap()),
        BufWriter::new(File::create(dest_file_path).unwrap()),
        password,
    )
    .unwrap();
    decrypt_file(
        BufReader::new(File::open(dest_file_path).unwrap()),
        BufWriter::new(File::create(decrypted_file_path).unwrap()),
        password,
    )
    .unwrap();
    assert_eq!(
        std::fs::read(source_file_path).unwrap(),
        std::fs::read(decrypted_file_path).unwrap()
    );
}
