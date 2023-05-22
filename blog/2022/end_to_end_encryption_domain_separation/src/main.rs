use blake2::digest::{Update, VariableOutput};
use rand::{rngs::OsRng, RngCore};
use zeroize::Zeroize;

const ENCRYPTION_DOMAIN: &str = "encryption";
const AUTH_DOMAIN: &str = "auth";
const KEY_SIZE: usize = 32;

fn main() -> Result<(), anyhow::Error> {
    let mut password = rpassword::prompt_password_stdout("password:")?;
    let argon2_config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    };

    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let mut master_key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let mut encryption_kdf = blake2::VarBlake2b::new_keyed(&master_key, KEY_SIZE);
    encryption_kdf.update(ENCRYPTION_DOMAIN.as_bytes());
    let mut encryption_key = encryption_kdf.finalize_boxed();

    let mut auth_kdf = blake2::VarBlake2b::new_keyed(&master_key, KEY_SIZE);
    auth_kdf.update(AUTH_DOMAIN.as_bytes());
    let mut auth_key = auth_kdf.finalize_boxed();

    println!("master key:     {}", hex::encode(&master_key));
    println!("encryption key: {}", hex::encode(&encryption_key));
    println!("auth key:       {}", hex::encode(&auth_key));

    password.zeroize();
    master_key.zeroize();
    encryption_key.zeroize();
    auth_key.zeroize();

    Ok(())
}
