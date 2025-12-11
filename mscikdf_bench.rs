use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mscikdf::{MSCIKDF, SignFlavor};
use mscikdf::{
    WeiToCryptoEntityUtil,
    LegacyPrivKeyType,
};
use uuid::Uuid;
use std::time::Duration;
use std::ffi::CString;
use std::ffi::CStr;

// --- Setup ---
// Pre-set immutable inputs to avoid repeated computation inside benchmarks
const MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const PASS: &str = "test_passphrase";
// Note: Keeping placeholder content generic, assuming 'filler' is descriptive
const MSG: &[u8] = b"transaction_data_to_sign_128_bytes_filler_filler_filler_filler_filler";

fn setup_signing() -> (String, &'static str) {
    let wallet = MSCIKDF::generate_internal("pass_for_bench", None);
    (wallet.mnemonic, "pass_for_bench")
}
fn setup() -> (Uuid, String) {
    // Use a fixed UUID for repeatable testing
    let fixed_uuid_bytes = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
        0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10
    ];
    let uuid = Uuid::from_bytes(fixed_uuid_bytes);

    // Generate a temporary mnemonic for performance testing of Derive and Sign
    let wallet = MSCIKDF::generate_internal(PASS, None);
    (uuid, wallet.mnemonic)
}

// --- Benchmarks ---

fn benchmark_core_operations(c: &mut Criterion) {
    // ------------------ Setup ------------------
    // Note: All required imports are now assumed to be at the top of the file.

    let (uuid, _) = setup();
    let (mn_sign, pass_sign) = setup_signing();
    // const MSG is defined globally

    // --- FFI Setup ---
    // Prepare CString for FFI calls; CString ensures zero-termination and manages memory automatically at the end of the function scope.
    let mn_cstring = CString::new(mn_sign.clone()).unwrap();
    let pass_cstring = CString::new(pass_sign).unwrap();

    // Get C pointers for FFI
    let mn_ptr = mn_cstring.as_ptr();
    let pass_ptr = pass_cstring.as_ptr();

    // --- XID Setup (Must be done outside the loop) ---
    // 1. Call FFI function to get the XID public key (requires unsafe block)
    let xid_pub: String = unsafe {
        // Explicitly cast to the required FFI pointer type: *const u8 -> *const c_char
        let xid_pub_c_ptr = MSCIKDF::get_xidentity(
            mn_ptr,
            pass_ptr,
            None
        );

        if xid_pub_c_ptr.is_null() {
            panic!("Failed to generate xidentity in benchmark setup.");
        }

        // 2. Safely convert the *mut c_char pointer to a Rust String
        let result = CStr::from_ptr(xid_pub_c_ptr).to_str().unwrap().to_string();

        // 3. CRITICAL: Free the memory allocated inside the FFI function (via CString::into_raw)
        MSCIKDF::free_string(xid_pub_c_ptr);

        result
    };
    // ------------------------------------

    let mut group = c.benchmark_group("MSCIKDF_CORE_PERFORMANCE");
    group.sample_size(100)
             .measurement_time(Duration::from_secs(20))
             .warm_up_time(Duration::from_secs(5));

    // =======================================================
    // A. Core Identity Derivation Speed (Context Isolation)
    //    Goal: Verify HKDF speed; should be extremely fast
    // =======================================================
    group.bench_function("A1. Derive_Triple_Seed", |b| {
        b.iter(|| MSCIKDF::derive_from_uuid(black_box(&uuid)))
    });

    // =======================================================
    // B. Signing Performance (Wallet Responsiveness)
    //    Goal: Verify core asset signing speed; should be constant-time and fast
    // =======================================================
    group.bench_function("B1. Ed25519_Sign (Solana/Polkadot)", |b| {
        b.iter(|| MSCIKDF::sign_message_internal(
            black_box(&mn_sign), black_box(pass_sign), black_box(MSG),
            SignFlavor::Ed25519Raw, None
        ))
    });

    group.bench_function("B2. Secp256k1_Sign (EVM)", |b| {
        b.iter(|| MSCIKDF::sign_message_internal(
            black_box(&mn_sign), black_box(pass_sign), black_box(MSG),
            SignFlavor::Secp256k1Evm, None
        ))
    });

    // =======================================================
    // C. Key Exchange and Decryption Performance (E2EE)
    //    Goal: Verify XID efficiency for real-time AI Agent communication
    // =======================================================
    group.bench_function("C1. X25519_DH_Compute (Shared Secret)", |b| {
        b.iter(|| MSCIKDF::compute_dh_key_internal(
            black_box(&mn_sign), black_box(pass_sign), black_box(&xid_pub), None
        ))
    });

    // =======================================================
    // D. Auxiliary Functions (Metadata/Mnemonic Encoding & Legacy Anchoring)
    //    Goal: Verify user interface and migration latency
    // =======================================================
    // Note: sealed_bytes is unused in the bench, but kept for clarity
    let sealed_bytes = MSCIKDF::decode_mnemonic_to_sealed(&mn_sign).unwrap();

    group.bench_function("D1. Mnemonic_Decode_Sealed", |b| {
        b.iter(|| MSCIKDF::decode_mnemonic_to_sealed(black_box(MNEMONIC)))
    });

    // D2. Legacy Key Anchoring: PK -> UUID (Anchor function only)
    let pk32 = [0x55; 32];
    group.bench_function("D2. Anchor_PK_to_UUID", |b| {
        b.iter(|| WeiToCryptoEntityUtil::anchor_pk_to_uuid(
            LegacyPrivKeyType::Secp256k1, black_box(&pk32)
        ))
    });


    group.finish();

    // CString memory is safely deallocated upon function exit.
}

criterion_group!(benches, benchmark_core_operations);
criterion_main!(benches);

