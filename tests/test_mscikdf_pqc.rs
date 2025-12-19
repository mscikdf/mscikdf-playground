// use mscikdf::{MSCIKDF, SignFlavor};
// use fips204::traits::SerDes;

#[cfg(feature = "pqc-sign")]
#[test]
fn test_pqc_sign_with_wrong_passphrase_must_fail() {
    use mscikdf::{MSCIKDF, SignFlavor};

    let wallet = MSCIKDF::generate_internal("correct-pass", None)
        .expect("generate_internal failed");

    let mnemonic = wallet.mnemonic.clone();
    let message = b"pqc wrong pass test";

    // Use wrong passphrase
    let res = MSCIKDF::sign_message_internal(
        &mnemonic,
        "wrong-pass",
        message,
        SignFlavor::MlDsa44Raw,
        None,
    );

    assert!(
        res.is_err(),
        "PQC signing with wrong passphrase must fail"
    );
}

#[cfg(feature = "pqc-sign")]
#[test]
fn test_ml_dsa_signature_determinism() {
    let wallet = MSCIKDF::generate_internal("pqc-test", None).unwrap();
    let msg = b"quantum-secure-message-2025";

    // Sign twice
    let sig1 = MSCIKDF::sign_message_internal(
        &wallet.mnemonic, "pqc-test", msg, SignFlavor::MlDsa44Raw, None
    ).unwrap();

    let sig2 = MSCIKDF::sign_message_internal(
        &wallet.mnemonic, "pqc-test", msg, SignFlavor::MlDsa44Raw, None
    ).unwrap();

    assert_eq!(sig1, sig2, "PQC signatures must be deterministic in this implementation");
    assert!(sig1.len() >= 2420, "ML-DSA-44 signature length check");
}

#[cfg(feature = "pqc-sign")]
#[test]
fn test_pqc_sign_with_secondary_factor() {
    use mscikdf::{MSCIKDF, SignFlavor};

    let primary = "primary-pass";
    let secondary = "enterprise-factor";

    let wallet = MSCIKDF::generate_internal(primary, Some(secondary))
        .expect("generate_internal failed");

    let mnemonic = wallet.mnemonic.clone();
    let message = b"pqc secondary test";

    // Correct secondary
    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        primary,
        message,
        SignFlavor::MlDsa44Raw,
        Some(secondary),
    )
    .expect("PQC sign with secondary failed");

    // Incorrect secondary
    let res = MSCIKDF::sign_message_internal(
        &mnemonic,
        primary,
        message,
        SignFlavor::MlDsa44Raw,
        Some("wrong-secondary"),
    );

    assert!(
        res.is_err(),
        "PQC signing with wrong secondary must fail"
    );

    assert!(
        sig.len() > 2000,
        "unexpected PQC signature length"
    );
}

#[cfg(feature = "pqc-sign")]
#[test]
fn test_ml_dsa_44_sign_and_verify() {
    use fips204::ml_dsa_44;
    use rand_chacha::ChaCha20Rng;
    use rand_core::SeedableRng;

    let wallet = MSCIKDF::generate_internal("test-pass", None).unwrap();
    let mnemonic = wallet.mnemonic;

    let passphrase = "test-pass";
    let message = b"hello pqc";

    // 1. Sign with MSCIKDF
    let sig = MSCIKDF::sign_message_internal(
        &mnemonic,
        passphrase,
        message,
        SignFlavor::MlDsa44Raw,
        None,
    ).expect("sign failed");

    let resolved = MSCIKDF::resolve_root(&mnemonic, passphrase, None).unwrap();
    let seeds = MSCIKDF::derive_from_uuid(&resolved.uuid).unwrap();

    let seed32: &[u8; 32] = seeds.ml_dsa_44[..].try_into().unwrap();
    let mut rng = ChaCha20Rng::from_seed(*seed32);
    let (pk, _sk) = ml_dsa_44::try_keygen_with_rng(&mut rng).unwrap();

    let pk_bytes = pk.into_bytes();

    // 3. verify
    let ok = MSCIKDF::verify_ml_dsa_44_signature(
        &pk_bytes,
        message,
        &sig,
    );

    assert!(ok);
}

#[cfg(feature = "pqc-sign")]
#[test]
fn test_xaddress_ownership_proof_roundtrip() {
    use rand::RngCore;
    use mscikdf::{MSCIKDF, SignFlavor};

    let passphrase = "xaddr-proof-pass";
    let secondary = Some("enterprise-factor");

    let wallet = MSCIKDF::generate_internal(passphrase, secondary)
        .expect("wallet generation failed");

    let mnemonic = wallet.mnemonic.clone();
    let xaddress = wallet.xaddress.clone();

    assert!(
        xaddress.starts_with("msci1"),
        "xaddress must be bech32m encoded"
    );

    let mut nonce = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut nonce);

    let challenge = {
        let mut msg = Vec::new();
        msg.extend_from_slice(b"MSCIKDF:XADDR:PROOF:v1|");
        msg.extend_from_slice(b"aud=test-suite|");
        msg.extend_from_slice(&nonce);
        msg
    };

    let pqc_pubkey = MSCIKDF::ml_dsa_44_pubkey_internal(
        &mnemonic,
        passphrase,
        secondary,
    )
    .expect("derive pqc pubkey failed");

    let signature = MSCIKDF::sign_message_internal(
        &mnemonic,
        passphrase,
        &challenge,
        SignFlavor::MlDsa44Raw,
        secondary,
    )
    .expect("pqc sign failed");

   let sig_ok = MSCIKDF::verify_ml_dsa_44_signature(
        &pqc_pubkey,
        &challenge,
        &signature,
    );

    assert!(sig_ok, "PQC signature verification must succeed");

    let (_version, fp_from_addr) =
        MSCIKDF::decode_xaddress(&xaddress)
            .expect("decode xaddress failed");

    let fp_from_pubkey =
        MSCIKDF::xaddress_fingerprint_from_pqc_pubkey(&pqc_pubkey);

    assert_eq!(
        fp_from_addr,
        fp_from_pubkey,
        "xaddress fingerprint must match pubkey-derived fingerprint"
    );
}
