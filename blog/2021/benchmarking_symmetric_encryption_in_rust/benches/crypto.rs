use aes_gcm::Aes256Gcm;
use chacha20::{cipher::KeyIvInit, cipher::StreamCipher, ChaCha20};
use chacha20poly1305::{
    aead::{AeadInPlace, NewAead},
    ChaCha20Poly1305, XChaCha20Poly1305,
};
use criterion::*;
use rand::RngCore;

fn bench(c: &mut Criterion) {
    let mut rand_generator = black_box(rand::rngs::OsRng {});
    let ad = [0u8; 32];
    let mut key = [0u8; 32];
    black_box(rand_generator.fill_bytes(&mut key));
    let mut nonce_96 = [0u8; 12];
    black_box(rand_generator.fill_bytes(&mut nonce_96));
    let mut nonce_192 = [0u8; 24];
    black_box(rand_generator.fill_bytes(&mut nonce_192));

    // 100B
    let mut in_out = black_box(vec![0u8; 100]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));
    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("100B");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });

    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad: ring::aead::Aad<&[u8; 32]> = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 1kB
    let mut in_out = black_box(vec![0u8; 1000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));
    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("1kB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });
    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 100kB
    let mut in_out = black_box(vec![0u8; 100_000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));
    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("100kB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });
    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 1MB
    let mut in_out = black_box(vec![0u8; 1_000_000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));

    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("1MB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });
    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 10MB
    let mut in_out = black_box(vec![0u8; 10_000_000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));

    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("10MB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });
    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 100MB
    let mut in_out = black_box(vec![0u8; 100_000_000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));

    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("100MB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });

    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();

    // 1GB
    let mut in_out = black_box(vec![0u8; 1_000_000_000]);
    let xchacha20_poly1305 = black_box(XChaCha20Poly1305::new(key.as_ref().into()));
    let chacha20_poly1305 = black_box(ChaCha20Poly1305::new(key.as_ref().into()));
    let mut chacha20 = black_box(ChaCha20::new(key.as_ref().into(), nonce_96.as_ref().into()));

    let aes_256_gcm = black_box(Aes256Gcm::new(key.as_ref().into()));
    let ring_key_chacha20 = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::CHACHA20_POLY1305, &key).unwrap(),
    ));
    let ring_key_aesgcm = black_box(ring::aead::LessSafeKey::new(
        ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, &key).unwrap(),
    ));

    let mut group = c.benchmark_group("1GB");
    group.throughput(Throughput::Bytes(in_out.len() as u64));
    group.bench_function("RustCrypto XChaCha20-Poly1305", |b| {
        b.iter(|| {
            xchacha20_poly1305.encrypt_in_place_detached(
                nonce_192.as_ref().into(),
                &ad,
                &mut in_out,
            )
        });
    });
    group.bench_function("RustCrypto ChaCha20-Poly1305", |b| {
        b.iter(|| {
            chacha20_poly1305.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("RustCrypto ChaCha20", |b| {
        b.iter(|| chacha20.apply_keystream(&mut in_out));
    });
    group.bench_function("RustCrypto AES-256-GCM", |b| {
        b.iter(|| {
            aes_256_gcm.encrypt_in_place_detached(nonce_96.as_ref().into(), &ad, &mut in_out)
        });
    });
    group.bench_function("ring ChaCha20-Poly1305", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_chacha20.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.bench_function("ring AES-256-GCM", |b| {
        b.iter(|| {
            let ring_nonce = ring::aead::Nonce::assume_unique_for_key(nonce_96);
            let ring_ad = ring::aead::Aad::from(black_box(&ad));
            ring_key_aesgcm.seal_in_place_separate_tag(ring_nonce, ring_ad, &mut in_out)
        });
    });
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
