use blake2::digest::{Update, VariableOutput};
use rand::{rngs::OsRng, RngCore};
use x25519_dalek::{PublicKey, StaticSecret};

const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;

fn main() {
    let mut rand_generator = OsRng {};

    let mut nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);

    let alice_private_key = StaticSecret::new(rand_generator);
    let alice_public_key = PublicKey::from(&alice_private_key);

    let bob_private_key = StaticSecret::new(rand_generator);
    let bob_public_key = PublicKey::from(&bob_private_key);

    let bob_secret = derive_secret_for_bob(&bob_private_key, &alice_public_key, &nonce);
    let alice_secret = derive_secret_for_alice(&alice_private_key, &bob_public_key, &nonce);

    assert!(bob_secret == alice_secret);

    println!("Everything is good!");
}

fn derive_secret_for_bob(
    bob_private_key: &StaticSecret,
    alice_public_key: &PublicKey,
    nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
) -> Vec<u8> {
    let dh_secret = bob_private_key.diffie_hellman(&alice_public_key);

    let mut kdf = blake2::VarBlake2b::new_keyed(dh_secret.as_bytes(), XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(nonce);
    let shared_key = kdf.finalize_boxed();

    return shared_key.into();
}

fn derive_secret_for_alice(
    alice_private_key: &StaticSecret,
    bob_public_key: &PublicKey,
    nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
) -> Vec<u8> {
    let dh_secret = alice_private_key.diffie_hellman(&bob_public_key);

    let mut kdf = blake2::VarBlake2b::new_keyed(dh_secret.as_bytes(), XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(nonce);
    let shared_key = kdf.finalize_boxed();

    return shared_key.into();
}
