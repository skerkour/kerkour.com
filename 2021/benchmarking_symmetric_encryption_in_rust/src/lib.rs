use chacha20poly1305::{aead::AeadInPlace, ChaCha20Poly1305, XChaCha20Poly1305};

pub fn encrypt_xchacha20_poly1305(
    cipher: &XChaCha20Poly1305,
    nonce: &[u8],
    ad: &[u8],
    data: &mut [u8],
) {
    let _tag = cipher.encrypt_in_place_detached(nonce.into(), ad, data);
}

pub fn encrypt_chacha20_poly1305(
    cipher: &ChaCha20Poly1305,
    nonce: &[u8],
    ad: &[u8],
    data: &mut [u8],
) {
    let _tag = cipher.encrypt_in_place_detached(nonce.into(), ad, data);
}
