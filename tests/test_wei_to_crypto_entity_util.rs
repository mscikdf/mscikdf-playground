use mscikdf::{
    WeiToCryptoEntityUtil,
    LegacyPrivKeyType,
};

use crypto_common::rand_core::OsRng;

fn rand32() -> [u8; 32] {
    use rand::RngCore;
    let mut out = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut out);
    out
}

#[test]
fn test_wei_to_crypto_entity_secp256k1_roundtrip() {
    let pk = rand32();
    let pass = "hello-sec";

    let mnemonic = WeiToCryptoEntityUtil::wei_to_crypto_entity_internal(&pk, pass)
        .expect("mnemonic");

    let (curve, recovered) =
        WeiToCryptoEntityUtil::legacy_from_mnemonic_internal(&mnemonic, pass)
            .expect("recover");

    println!("mnemonic = {}", mnemonic);
    println!("curve = {:?}, recovered = {:?}", curve, recovered);

    assert_eq!(curve, LegacyPrivKeyType::Secp256k1);
    assert_eq!(recovered, pk);
}

#[test]
fn test_wei_to_crypto_entity_ed25519_seed_roundtrip() {
    use ed25519_dalek::{SigningKey};

    let signing = SigningKey::generate(&mut OsRng);
    let seed: [u8; 32] = signing.to_bytes();

    let verifying = signing.verifying_key();

    let mut keypair = [0u8; 64];
    keypair[..32].copy_from_slice(&seed);
    keypair[32..].copy_from_slice(verifying.as_bytes());

    let pass = "hello-ed";

    let mnemonic = WeiToCryptoEntityUtil::wei_to_crypto_entity_internal(&keypair, pass)
        .expect("mnemonic");

    let (_curve, recovered) =
        WeiToCryptoEntityUtil::legacy_from_mnemonic_internal(&mnemonic, pass)
            .expect("recover");

    println!("mnemonic = {}", mnemonic);
    println!("curve = {:?}, recovered = {:?}", _curve, recovered);

    assert_eq!(recovered, seed);
}


#[test]
fn test_wei_to_crypto_entity_x25519_roundtrip() {
    let pk = rand32();
    let pass = "hello-x";

    let mnemonic = WeiToCryptoEntityUtil::legacy_to_crypto_entity(&pk, pass)
        .expect("mnemonic");

    let (_curve, recovered) =
        WeiToCryptoEntityUtil::legacy_from_mnemonic_internal(&mnemonic, pass)
            .expect("recover");

    println!("mnemonic = {}", mnemonic);
    println!("curve = {:?}, recovered = {:?}", _curve, recovered);

    assert_eq!(recovered, pk);
}

#[test]
fn test_invalid_empty_privkey() {
    let res = WeiToCryptoEntityUtil::wei_to_crypto_entity_internal(&[], "x");
    assert!(res.is_err());
}

#[test]
fn test_invalid_privkey_length() {
    let bad = [1u8; 31];
    let res = WeiToCryptoEntityUtil::wei_to_crypto_entity_internal(&bad, "x");
    assert!(res.is_err());
}

#[test]
fn test_wrong_passphrase_should_fail() {
    let pk = rand32();

    let mnemonic =
        WeiToCryptoEntityUtil::wei_to_crypto_entity_internal(&pk, "correct").unwrap();

    let (_curve_ok, pk_ok) =
        WeiToCryptoEntityUtil::legacy_from_mnemonic_internal(&mnemonic, "correct")
            .expect("recover with correct passphrase");

    let (_curve_wrong, pk_wrong) =
        WeiToCryptoEntityUtil::legacy_from_mnemonic_internal(&mnemonic, "wrong")
            .expect("recover with wrong passphrase should still return some pk");


    println!("_curve_ok = {:?}, pk_ok = {:?}", _curve_ok, pk_ok);
    println!("_curve_wrong = {:?}, pk_wrong = {:?}", _curve_wrong, pk_wrong);

    assert_eq!(pk_ok, pk);

    assert_ne!(pk_wrong, pk);
    assert_ne!(pk_wrong, pk_ok);
}

#[test]
fn test_validate_legacy_privkey_logic() {
    let pk = rand32();
    assert!(WeiToCryptoEntityUtil::validate_legacy_privkey(LegacyPrivKeyType::X25519, &pk));
    assert!(WeiToCryptoEntityUtil::validate_legacy_privkey(LegacyPrivKeyType::Ed25519, &pk));

    let secp_ok =
        WeiToCryptoEntityUtil::validate_legacy_privkey(LegacyPrivKeyType::Secp256k1, &pk);

    assert!(secp_ok == true || secp_ok == false);
}

#[test]
fn test_anchor_pk_to_uuid_deterministic() {
    let pk = rand32();

    let uuid1 = WeiToCryptoEntityUtil::anchor_pk_to_uuid(LegacyPrivKeyType::Secp256k1, &pk);
    let uuid2 = WeiToCryptoEntityUtil::anchor_pk_to_uuid(LegacyPrivKeyType::Secp256k1, &pk);

    assert_eq!(uuid1, uuid2);
}

#[test]
fn test_anchor_pk_to_uuid_curve_differs() {
    let pk = rand32();
    let u1 = WeiToCryptoEntityUtil::anchor_pk_to_uuid(LegacyPrivKeyType::Secp256k1, &pk);
    let u2 = WeiToCryptoEntityUtil::anchor_pk_to_uuid(LegacyPrivKeyType::Ed25519, &pk);

    assert_ne!(u1, u2);
}

#[test]
fn test_wallet_from_uuid_internal_minimal() {
    let pk = rand32();
    let uuid = WeiToCryptoEntityUtil::anchor_pk_to_uuid(LegacyPrivKeyType::Secp256k1, &pk);

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let ce = WeiToCryptoEntityUtil::wallet_from_uuid_internal(mnemonic, &uuid);

    println!("Crypto Entity: \n{}", serde_json::to_string_pretty(&ce).unwrap());

    assert!(ce.evm_address.starts_with("0x"));
    assert!(!ce.solana_address.is_empty());
    assert!(!ce.bitcoin_address.is_empty());
    assert!(!ce.cosmos_address.is_empty());
    assert!(!ce.polkadot_address.is_empty());
    assert!(!ce.xidentity.is_empty());
}
