// tests/test_mscikdf.rs

use mscikdf::{MSCIKDF, SignFlavor};
use mscikdf::passphrase_sealing_util::{MscikdfError, PassphraseSealingUtil};
use base64ct::{Base64, Encoding};
use rand::rngs::OsRng;
use x25519_dalek::{StaticSecret, PublicKey as XPublic};
use sha3::Digest;
use arrayref::array_ref;
use uuid::Uuid;
 use rand::RngCore;

const PASS: &str = "unit_test_passphrase";
const MSG: &[u8] = b"hello world TEST MESSAGE";

#[test]
fn test_generate_and_view() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic;
    assert!(!mnemonic.is_empty());

    let view = MSCIKDF::view_wallet_internal(&mnemonic, PASS, None);
    assert!(view.solana_address.len() > 30);
    assert!(view.evm_address.starts_with("0x"));
}

#[test]
fn test_export_private_keys() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic;

    let keys = MSCIKDF::export_private_keys_internal(&mnemonic, PASS, None);
    assert_eq!(keys.mnemonic, mnemonic);
    assert!(keys.evm_private_key.starts_with("0x"));
}

#[test]
fn test_generate_and_view_with_secondary() {
    let primary = "prim_pass";
    let secondary = "sec_enterprise";

    let wallet = MSCIKDF::generate_internal(primary, Some(secondary)).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let view_ok = MSCIKDF::view_wallet_internal(&mnemonic, primary, Some(secondary));
    assert!(view_ok.evm_address.starts_with("0x"));
    assert!(!view_ok.solana_address.is_empty());

    let view_fail = MSCIKDF::view_wallet_internal(&mnemonic, primary, None);
    assert!(view_fail.evm_address.is_empty());
    assert!(view_fail.solana_address.is_empty());

    let view_fail2 =
        MSCIKDF::view_wallet_internal(&mnemonic, primary, Some("wrong-secondary"));
    assert!(view_fail2.evm_address.is_empty());
    assert!(view_fail2.solana_address.is_empty());
}

#[test]
fn test_export_private_keys_with_secondary() {
    let primary = "pp_with_sec";
    let secondary = "sec_layer";

    let wallet = MSCIKDF::generate_internal(primary, Some(secondary)).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let keys_ok =
        MSCIKDF::export_private_keys_internal(&mnemonic, primary, Some(secondary));
    assert_eq!(keys_ok.mnemonic, mnemonic);
    assert!(keys_ok.evm_private_key.starts_with("0x"));

    let res = std::panic::catch_unwind(|| {
        MSCIKDF::export_private_keys_internal(&mnemonic, primary, None)
    });
    assert!(res.is_err(), "missing secondary must not decrypt");
}

#[test]
fn test_sign_secp256k1() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic;

    let sig = MSCIKDF::sign_message_internal(&mnemonic, PASS, MSG, SignFlavor::Secp256k1Evm, None).unwrap();
    assert_eq!(sig.len(), 64);
}

#[test]
fn test_xid_encrypt_decrypt_via_dh() {
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
    use rand::RngCore;

    let wallet_a = MSCIKDF::generate_internal("aaa_pass", None).unwrap();
    let wallet_b = MSCIKDF::generate_internal("bbb_pass", None).unwrap();

    let mn_a = wallet_a.mnemonic.clone();
    let mn_b = wallet_b.mnemonic.clone();

    let xid_a = wallet_a.xidentity.clone();
    let xid_b = wallet_b.xidentity.clone();

    let shared_ab = MSCIKDF::compute_dh_key_internal(&mn_a, "aaa_pass", &xid_b, None).unwrap();
    let shared_ba = MSCIKDF::compute_dh_key_internal(&mn_b, "bbb_pass", &xid_a, None).unwrap();
    assert_eq!(shared_ab, shared_ba);

    let eph_secret = StaticSecret::random_from_rng(OsRng);
    let eph_public = XPublic::from(&eph_secret);
    let eph_pub_b64 = Base64::encode_string(eph_public.as_bytes());

    let peer_b_pub_bytes = Base64::decode_vec(&xid_b).unwrap();
    let peer_b_pub = XPublic::from(*array_ref![peer_b_pub_bytes, 0, 32]);

    let master_shared = eph_secret.diffie_hellman(&peer_b_pub);
    let master_hash = sha2::Sha256::digest(master_shared.as_bytes());
    let master_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&master_hash[..32]);
    let cipher_master = Aes256Gcm::new(master_key);

    let mut real_key = [0u8; 32];
    OsRng.fill_bytes(&mut real_key);

    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);

    let encrypted_aes_key = cipher_master.encrypt(
        Nonce::from_slice(&iv),
        &real_key[..]
    ).unwrap();

    let encrypted_aes_key_b64 = Base64::encode_string(&encrypted_aes_key);
    let iv_b64 = Base64::encode_string(&iv);

    let cipher_real = Aes256Gcm::new(aes_gcm::Key::<Aes256Gcm>::from_slice(&real_key));
    let plaintext = b"Hello B, msg encrypted using XID DH!";
    let encrypted_data = cipher_real.encrypt(
        Nonce::from_slice(&iv),
        plaintext.as_ref(),
    ).unwrap();

    let decrypted = MSCIKDF::decrypt_internal(
        &mn_b,
        "bbb_pass",
        &eph_pub_b64,
        &encrypted_aes_key_b64,
        &iv_b64,
        &encrypted_data,
        None,
    ).unwrap();

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_xid_dh_and_decrypt_with_secondary() {
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
    use rand::RngCore;

    let primary_a = "aaa_prim";
    let primary_b = "bbb_prim";
    let sec = "enterprise_factor";

    let wallet_a = MSCIKDF::generate_internal(primary_a, Some(sec)).unwrap();
    let wallet_b = MSCIKDF::generate_internal(primary_b, Some(sec)).unwrap();

    let mn_a = wallet_a.mnemonic.clone();
    let mn_b = wallet_b.mnemonic.clone();

    let xid_a = wallet_a.xidentity.clone();
    let xid_b = wallet_b.xidentity.clone();

    let shared_ab =
        MSCIKDF::compute_dh_key_internal(&mn_a, primary_a, &xid_b, Some(sec)).unwrap();
    let shared_ba =
        MSCIKDF::compute_dh_key_internal(&mn_b, primary_b, &xid_a, Some(sec)).unwrap();
    assert_eq!(shared_ab, shared_ba);

    let eph_secret = StaticSecret::random_from_rng(OsRng);
    let eph_public = XPublic::from(&eph_secret);
    let eph_pub_b64 = Base64::encode_string(eph_public.as_bytes());

    let peer_b_pub_bytes = Base64::decode_vec(&xid_b).unwrap();
    let peer_b_pub = XPublic::from(*arrayref::array_ref![peer_b_pub_bytes, 0, 32]);

    let master_shared = eph_secret.diffie_hellman(&peer_b_pub);
    let master_hash = sha2::Sha256::digest(master_shared.as_bytes());
    let master_key = aes_gcm::Key::<Aes256Gcm>::from_slice(&master_hash[..32]);
    let cipher_master = Aes256Gcm::new(master_key);

    let mut real_key = [0u8; 32];
    OsRng.fill_bytes(&mut real_key);

    let mut iv = [0u8; 12];
    OsRng.fill_bytes(&mut iv);

    let encrypted_aes_key = cipher_master
        .encrypt(Nonce::from_slice(&iv), &real_key[..])
        .unwrap();

    let encrypted_aes_key_b64 = Base64::encode_string(&encrypted_aes_key);
    let iv_b64 = Base64::encode_string(&iv);

    let cipher_real = Aes256Gcm::new(aes_gcm::Key::<Aes256Gcm>::from_slice(&real_key));
    let plaintext = b"Hello with secondary!";
    let encrypted_data = cipher_real
        .encrypt(Nonce::from_slice(&iv), plaintext.as_ref())
        .unwrap();

    let decrypted = MSCIKDF::decrypt_internal(
        &mn_b,
        primary_b,
        &eph_pub_b64,
        &encrypted_aes_key_b64,
        &iv_b64,
        &encrypted_data,
        Some(sec),
    )
    .unwrap();

    assert_eq!(decrypted, plaintext);

    let res_fail = MSCIKDF::decrypt_internal(
        &mn_b,
        primary_b,
        &eph_pub_b64,
        &encrypted_aes_key_b64,
        &iv_b64,
        &encrypted_data,
        None,
    );
    assert!(res_fail.is_err(), "missing secondary must fail to decrypt");
}


#[test]
fn test_rotate_passphrase() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let new_pass = "my_new_pass_999";

    let new_mnemonic =
        MSCIKDF::change_passphrase_internal(&mnemonic, PASS, new_pass, None).unwrap();

    assert_ne!(mnemonic, new_mnemonic, "Mnemonic should change after rotation");

    let sealed_old = MSCIKDF::decode_mnemonic_to_sealed(&mnemonic).unwrap();
    let sealed_new = MSCIKDF::decode_mnemonic_to_sealed(&new_mnemonic).unwrap();

    let uuid_old =
        PassphraseSealingUtil::unseal(&sealed_old, PASS.as_bytes()).expect("old pass should work");

    let res_old_on_new = PassphraseSealingUtil::unseal(&sealed_new, PASS.as_bytes());
    assert!(
        matches!(res_old_on_new, Err(MscikdfError::InvalidPassphrase)),
        "old passphrase must NOT be able to unseal new sealed entropy"
    );

    let uuid_new =
        PassphraseSealingUtil::unseal(&sealed_new, new_pass.as_bytes()).expect("new pass must work");

    assert_eq!(
        uuid_new, uuid_old,
        "rotation should preserve the underlying UUID (root identity)"
    );
}
#[test]
fn test_rotate_passphrase_with_secondary() {
    let primary = "rotate_prim";
    let secondary = "rotate_sec";

    let wallet = MSCIKDF::generate_internal(primary, Some(secondary)).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let new_pass = "rotate_new_123";

    let new_mnemonic = MSCIKDF::change_passphrase_internal(
        &mnemonic,
        primary,
        new_pass,
        Some(secondary),
    )
    .unwrap();

    assert_ne!(mnemonic, new_mnemonic);

    let sealed_old = MSCIKDF::decode_mnemonic_to_sealed(&mnemonic).unwrap();
    let sealed_new = MSCIKDF::decode_mnemonic_to_sealed(&new_mnemonic).unwrap();

    let uuid_old = PassphraseSealingUtil::unseal(
        &sealed_old,
        MSCIKDF::combine_passphrase(primary, Some(secondary)).as_bytes(),
    )
    .expect("old pass+secondary should work");

    let res_old_on_new = PassphraseSealingUtil::unseal(
        &sealed_new,
        MSCIKDF::combine_passphrase(primary, Some(secondary)).as_bytes(),
    );
    assert!(
        matches!(res_old_on_new, Err(MscikdfError::InvalidPassphrase)),
        "old passphrase+secondary must NOT unseal new mnemonic"
    );

    let uuid_new = PassphraseSealingUtil::unseal(
        &sealed_new,
        MSCIKDF::combine_passphrase(new_pass, Some(secondary)).as_bytes(),
    )
    .expect("new pass+secondary should work");

    assert_eq!(uuid_old, uuid_new);
}

#[test]
fn test_verify_ed25519_signature_internal() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let (_sk, pk) = MSCIKDF::solana_keypair_internal(&mnemonic, PASS, None)
        .expect("solana_keypair_internal failed");

    let pub_bytes = pk.to_bytes();
    let ed25519_pub_b64 = Base64::encode_string(&pub_bytes);

    let message = b"test-verify-basic";
    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        PASS,
        message,
        SignFlavor::Ed25519Raw, None,
    )
    .expect("sign_message_internal failed");

    assert_eq!(sig.len(), 64);

    let ok = MSCIKDF::verify_ed25519_signature_internal(
        &ed25519_pub_b64,
        message,
        &sig,
    );

    assert!(ok, "verify_ed25519_signature_internal should accept a valid Ed25519 signature");
}


#[test]
fn test_verify_ed25519_signature_with_timestamp_internal() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let (_sk, pk) = MSCIKDF::solana_keypair_internal(&mnemonic, PASS, None)
        .expect("solana_keypair_internal failed");

    let pub_bytes = pk.to_bytes();
    let ed25519_pub_b64 = Base64::encode_string(&pub_bytes);

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let payload = "hello-ts";
    let message = format!("{}:{}", ts, payload);

    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        PASS,
        message.as_bytes(),
        SignFlavor::Ed25519Raw,
        None,
    )
    .expect("sign_message_internal failed");

    let ok = MSCIKDF::verify_ed25519_with_timestamp_internal(
        &ed25519_pub_b64,
        &message,
        &sig,
        60,
    );

    assert!(ok, "timestamp verify should succeed within allowed age");
}

#[test]
fn test_ed25519_sign_verify_with_secondary() {
    let primary = "sig_primary";
    let secondary = "sig_secondary";

    let wallet = MSCIKDF::generate_internal(primary, Some(secondary)).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let (_sk, pk) = MSCIKDF::solana_keypair_internal(
        &mnemonic,
        primary,
        Some(secondary),
    )
    .expect("solana_keypair_internal failed");

    let pub_bytes = pk.to_bytes();
    let ed25519_pub_b64 = Base64::encode_string(&pub_bytes);

    let message = b"sec-pass-sign";
    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        primary,
        message,
        SignFlavor::Ed25519Raw,
        Some(secondary),
    )
    .expect("sign_message_internal failed");

    assert_eq!(sig.len(), 64);

    let ok = MSCIKDF::verify_ed25519_signature_internal(
        &ed25519_pub_b64,
        message,
        &sig,
    );
    assert!(ok);

    let res_wrong_sec = MSCIKDF::sign_message_internal(
        &mnemonic,
        primary,
        message,
        SignFlavor::Ed25519Raw,
        Some("wrong-secondary"),
    );

    assert!(
        res_wrong_sec.is_err(),
        "signing with wrong secondary must fail"
    );
}

#[test]
fn test_verify_ed25519_signature_prefixed_simulated() {
    let wallet = MSCIKDF::generate_internal(PASS, None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let (_sk, pk) = MSCIKDF::solana_keypair_internal(&mnemonic, PASS, None)
        .expect("solana_keypair_internal failed");

    let pub_bytes = pk.to_bytes();
    let ed25519_pub_b64 = Base64::encode_string(&pub_bytes);

    let prefix = b"\x19Ethereum Signed Message:\n3";
    let msg = b"abc";

    let mut full = Vec::new();
    full.extend_from_slice(prefix);
    full.extend_from_slice(msg);

    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        PASS,
        &full,
        SignFlavor::Ed25519Raw,
        None,
    )
    .expect("sign_message_internal failed");

    let ok = MSCIKDF::verify_ed25519_signature_internal(
        &ed25519_pub_b64,
        &full,
        &sig,
    );

    assert!(ok, "prefixed Ed25519 signature should be verified correctly");
}

#[test]
fn test_resolve_root_uuid_stable_for_same_inputs() {
    let w = MSCIKDF::generate_internal("pass", Some("sec")).unwrap();
    let mn = w.mnemonic;

    let r1 = MSCIKDF::resolve_root(&mn, "pass", Some("sec")).unwrap();
    let r2 = MSCIKDF::resolve_root(&mn, "pass", Some("sec")).unwrap();

    assert_eq!(r1.uuid, r2.uuid);
}

#[test]
fn test_cryptographic_golden_vectors_and_domain_separation() {
    let uuid_str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";
    let uuid = Uuid::parse_str(uuid_str).unwrap();
    let pass = "mscikdf-golden-pass-2025";

    let seeds = MSCIKDF::derive_from_uuid(&uuid).unwrap();

    assert_eq!(hex::encode(&seeds.ed25519[..4]), "eac16cb8");
    assert_eq!(hex::encode(&seeds.secp256k1[..4]), "45a19328");
    assert_eq!(hex::encode(&seeds.ml_dsa_44[..4]), "639505b9");

    assert_ne!(seeds.ed25519, seeds.secp256k1, "Ed25519 and Secp256k1 seeds must be isolated");
    assert_ne!(seeds.x25519, seeds.ml_dsa_44, "X25519 and PQC seeds must be isolated");

    let sealed = PassphraseSealingUtil::seal(&uuid, pass).unwrap();
    let unsealed_uuid = PassphraseSealingUtil::unseal(&sealed, pass).unwrap();
    assert_eq!(uuid, unsealed_uuid, "Roundtrip must preserve UUID");

    let mut tampered_sealed = sealed.clone();
    tampered_sealed[31] ^= 0x01;
    let tamper_res = PassphraseSealingUtil::unseal(&tampered_sealed, pass);
    assert!(tamper_res.is_err(), "AES-SIV authentication tag must catch tampering");
}

#[test]
fn test_memory_zeroization_security_audit() {
    let mut sensitive_data = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut sensitive_data);

    let is_all_zero_init = sensitive_data.iter().all(|&b| b == 0);
    assert!(!is_all_zero_init, "Initial random data should not be all zeros");

    let ptr = sensitive_data.as_ptr();

    MSCIKDF::zero_out(&mut sensitive_data);

    unsafe {
        for i in 0..32 {
            let val = std::ptr::read_volatile(ptr.add(i));
            assert_eq!(val, 0, "Memory leak detected at byte [{}]: sensitive data was not zeroed!", i);
        }
    }

    let finalized_data = std::hint::black_box(sensitive_data);
    assert!(finalized_data.iter().all(|&b| b == 0), "Finalized data buffer must be all zeros");
}