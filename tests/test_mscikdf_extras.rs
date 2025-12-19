// tests/test_mscikdf_extras.rs

use mscikdf::{MSCIKDF, SignFlavor, CurveFamily, AddressFormat};
use base64ct::{Base64, Encoding};
use crypto_common::rand_core::{OsRng, RngCore};
use uuid::Uuid;

// ===============================================================
// 1. derive_from_uuid
// ===============================================================
#[test]
fn test_derive_from_uuid() {
    let uuid = Uuid::new_v4();
    let seeds = MSCIKDF::derive_from_uuid(&uuid).unwrap();

    assert_eq!(seeds.ed25519.len(), 32);
    assert_eq!(seeds.secp256k1.len(), 32);
    assert_eq!(seeds.x25519.len(), 32);
}

// ===============================================================
// 2. resolve_root ● native only
// ===============================================================
#[test]
fn test_resolve_root_native() {
    let wallet = MSCIKDF::generate_internal("pass", None).unwrap();
    let mnemonic = wallet.mnemonic;

    let resolved = MSCIKDF::resolve_root(&mnemonic, "pass", None).unwrap();

    // native path must return Native
    match resolved.source {
        mscikdf::RootSource::Native { .. } => {}
        _ => panic!("expected native root"),
    }
}

// ===============================================================
// 3. x25519_public_base64
// ===============================================================
#[test]
fn test_x25519_public_base64() {
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);

    let pub_b64 = MSCIKDF::x25519_public_base64(&seed).unwrap();
    let decoded = Base64::decode_vec(&pub_b64).unwrap();

    assert_eq!(decoded.len(), 32);
}

// ===============================================================
// 4. address_from_seed: EVM / BTC / Cosmos / SS58
// ===============================================================
#[test]
fn test_address_from_seed_all_formats() {
    // secp256k1 seed
    let mut s = [0u8; 32];
    OsRng.fill_bytes(&mut s);

    // EVM
    let evm = MSCIKDF::address_from_seed(&s, CurveFamily::Secp256k1, AddressFormat::Hex0x, None)
        .unwrap();
    assert!(evm.starts_with("0x") && evm.len() == 42);

    // BTC P2WPKH
    let btc = MSCIKDF::address_from_seed(
        &s,
        CurveFamily::Secp256k1,
        AddressFormat::Bech32,
        Some("bc"),
    )
    .unwrap();
    assert!(btc.starts_with("bc1"));

    // Cosmos
    let cosmos = MSCIKDF::address_from_seed(
        &s,
        CurveFamily::Secp256k1,
        AddressFormat::Bech32,
        Some("cosmos"),
    )
    .unwrap();
    assert!(cosmos.starts_with("cosmos1"));

    // ED25519 SS58
    let mut ed = [0u8; 32];
    OsRng.fill_bytes(&mut ed);

    let ss58 = MSCIKDF::address_from_seed(
        &ed,
        CurveFamily::Ed25519,
        AddressFormat::SS58,
        None,
    )
    .unwrap();
    assert!(ss58.len() > 40);
}

// ===============================================================
// 5. Private key raw extract functions
// ===============================================================
#[test]
fn test_private_key_extractors() {
    let mut seed = [0u8; 32];
    OsRng.fill_bytes(&mut seed);

    let evm = MSCIKDF::evm_private_key_from_seed(&seed);
    assert!(evm.starts_with("0x") && evm.len() == 66);

    let wif = MSCIKDF::bitcoin_wif_from_seed(&seed);
    assert!(wif.starts_with("K") || wif.starts_with("L"));

    let cosmos = MSCIKDF::cosmos_private_key_from_seed(&seed);
    assert_eq!(cosmos.len(), 64);
}

// ===============================================================
// 6. decode_mnemonic_to_sealed
// ===============================================================
#[test]
fn test_decode_mnemonic_to_sealed() {
    let wallet = MSCIKDF::generate_internal("ppp", None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let sealed = MSCIKDF::decode_mnemonic_to_sealed(&mnemonic).unwrap();
    assert_eq!(sealed.len(), 32);

    // invalid format
    let bad = MSCIKDF::decode_mnemonic_to_sealed("not a mnemonic");
    assert!(bad.is_err());
}

// ===============================================================
// 7. combine_passphrase
// ===============================================================
#[test]
fn test_combine_passphrase() {
    let a = MSCIKDF::combine_passphrase("abc", None);
    assert_eq!(a, "abc");

    let b = MSCIKDF::combine_passphrase("abc", Some("xyz"));
    assert_eq!(b, "abc\0xyz");
}

// ===============================================================
// 8. normalize_secp256k1_key
// ===============================================================
#[test]
fn test_normalize_secp256k1_key() {
    let seed = [1u8; 32];
    let normalized = MSCIKDF::normalize_secp256k1_key(&seed);

    // must not be zero, must be < curve order, but simplest check:
    assert_ne!(normalized, [0u8; 32]);
}

// ===============================================================
// 9. free_string
// ===============================================================
#[test]
fn test_free_string() {
    use std::ffi::CString;

    let raw = CString::new("hello").unwrap().into_raw();
    MSCIKDF::free_string(raw);
    // test passes if no crash / double free
}

// ===============================================================
// 10. sign_message_internal: Bitcoin / Cosmos Amino / Cosmos EIP712
// ===============================================================
#[test]
fn test_sign_secp256k1_bitcoin() {
    let wallet = MSCIKDF::generate_internal("pp", None).unwrap();
    let mn = wallet.mnemonic;

    let sig = MSCIKDF::sign_message_internal(
        &mn,
        "pp",
        b"bitcoin-msg",
        SignFlavor::Secp256k1Bitcoin,
        None,
    )
    .unwrap();

    assert_eq!(sig.len(), 64);
}

#[test]
fn test_sign_secp256k1_cosmos_amino() {
    let wallet = MSCIKDF::generate_internal("pp2", None).unwrap();
    let mn = wallet.mnemonic;

    let sig = MSCIKDF::sign_message_internal(
        &mn,
        "pp2",
        b"amino",
        SignFlavor::Secp256k1CosmosAmino,
        None,
    )
    .unwrap();

    assert_eq!(sig.len(), 64);
}

#[test]
fn test_sign_secp256k1_eip712() {
    let wallet = MSCIKDF::generate_internal("pp3", None).unwrap();
    let mn = wallet.mnemonic;

    let sig = MSCIKDF::sign_message_internal(
        &mn,
        "pp3",
        b"EIP712-test",
        SignFlavor::Secp256k1Eip712,
        None,
    )
    .unwrap();

    assert_eq!(sig.len(), 64);
}

// ===============================================================
// 11. sign_recoverable_internal (EVM style 65 bytes)
// ===============================================================
#[test]
fn test_sign_recoverable_internal() {
    let wallet = MSCIKDF::generate_internal("pp4", None).unwrap();
    let mn = wallet.mnemonic;

    let sig = MSCIKDF::sign_recoverable_internal(
        &mn,
        "pp4",
        b"recoverable",
        SignFlavor::Secp256k1Evm,
        None,
    )
    .unwrap();

    assert_eq!(sig.len(), 65);
    let v = sig[64];
    assert!(v == 27 || v == 28);
}

// ===============================================================
// 12. sign_secp256k1_recoverable_internal (r, s, v)
// ===============================================================
#[test]
fn test_sign_secp256k1_recoverable_internal() {
    let wallet = MSCIKDF::generate_internal("pp5", None).unwrap();
    let mn = wallet.mnemonic;

    let (r, s, v) =
        MSCIKDF::sign_secp256k1_recoverable_internal(&mn, "pp5", b"hello", None).unwrap();

    assert_eq!(r.len(), 32);
    assert_eq!(s.len(), 32);
    assert!(v == 0 || v == 1);
}

// ===============================================================
// 13. compute_dh_key_internal (standalone)
// ===============================================================
#[test]
fn test_compute_dh_key_internal_simple() {
    let wallet_a = MSCIKDF::generate_internal("aaa", None).unwrap();
    let wallet_b = MSCIKDF::generate_internal("bbb", None).unwrap();

    let mn_a = wallet_a.mnemonic;
    let mn_b = wallet_b.mnemonic;

    let xid_a = wallet_a.xidentity;
    let xid_b = wallet_b.xidentity;

    let ab = MSCIKDF::compute_dh_key_internal(&mn_a, "aaa", &xid_b, None).unwrap();
    let ba = MSCIKDF::compute_dh_key_internal(&mn_b, "bbb", &xid_a, None).unwrap();

    assert_eq!(ab, ba);
}
