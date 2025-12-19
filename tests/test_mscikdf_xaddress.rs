use mscikdf::{MSCIKDF};

#[test]
fn test_xaddress_encode_decode_roundtrip() {
    let fake_identity = vec![42u8; 32];
    let addr = MSCIKDF::encode_xaddress(&fake_identity, 0x01);

    let (version, fp) = MSCIKDF::decode_xaddress(&addr).unwrap();

    assert_eq!(version, 0x01);
    assert_eq!(fp.len(), 20);
}

#[test]
fn test_xaddress_reject_wrong_hrp() {
    let bad = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kygt080";
    assert!(MSCIKDF::decode_xaddress(bad).is_err());
}

#[test]
fn test_xaddress_reject_truncated() {
    let addr = "msci1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq";
    assert!(MSCIKDF::decode_xaddress(addr).is_err());
}


#[test]
fn test_generated_pqc_xaddress_is_decodable() {
    let wallet = MSCIKDF::generate_internal("pqc_test_pass", None).unwrap();

    let xaddr = wallet.xaddress.clone();
    assert!(xaddr.starts_with("msci"));

    let (version, fingerprint) = MSCIKDF::decode_xaddress(&xaddr).unwrap();


    assert_eq!(version, 0x01);

    assert_eq!(fingerprint.len(), 20);
}

#[test]
fn test_xaddress_stable_for_same_identity() {
    let wallet = MSCIKDF::generate_internal("stable_pass", None).unwrap();
    let mnemonic = wallet.mnemonic.clone();

    let view1 = MSCIKDF::view_wallet_internal(&mnemonic, "stable_pass", None);
    let view2 = MSCIKDF::view_wallet_internal(&mnemonic, "stable_pass", None);

    assert_eq!(view1.xaddress, view2.xaddress);

    let (_, fp1) = MSCIKDF::decode_xaddress(&view1.xaddress).unwrap();
    let (_, fp2) = MSCIKDF::decode_xaddress(&view2.xaddress).unwrap();

    assert_eq!(fp1, fp2);
}
