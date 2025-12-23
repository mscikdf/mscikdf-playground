use libloading::{Library, Symbol};
use std::env;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

mod abi;
use abi::{
    ACEGF_Call,
    ACEGF_Result,
    ACEGF_METHOD_GENERATE,
    ACEGF_METHOD_WEITOA,
    ACEGF_METHOD_REKEY,
    ACEGF_METHOD_VIEW,
};

/// -------- CLI-side JSON structs --------

#[derive(Deserialize, Serialize, Debug)]
pub struct CryptoEntity {
    pub mnemonic: String,
    pub solana_address: String,
    pub evm_address: String,
    pub bitcoin_address: String,
    pub cosmos_address: String,
    pub polkadot_address: String,
    pub xaddress: String,
    pub xidentity: String,
}

/// -------- Resolve dynamic library --------

fn resolve_library_path() -> PathBuf {
    let exe = env::current_exe().expect("Cannot get current executable path");
    let cli_dir = exe.parent().expect("Executable has no parent directory");
    let root = cli_dir.parent().expect("CLI dir has no parent");

    #[cfg(target_os = "linux")]
    {
        #[cfg(target_arch = "x86_64")]
        let arch = "linux/x86_64";
        #[cfg(target_arch = "aarch64")]
        let arch = "linux/arm64";

        root.join("lib").join(arch).join("libacegf.so")
    }

    #[cfg(target_os = "macos")]
    {
        root.join("lib/macos/libacegf.dylib")
    }
}

/// -------- Pretty printer --------

fn print_wallet(w: &CryptoEntity) {
    let styled_xaddress = format!("🐈{}", w.xaddress);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    println!("{{");
    println!("  \"mnemonic\": \"{}\",", w.mnemonic);
    println!("  \"solana_address\": \"{}\",", w.solana_address);
    println!("  \"evm_address\": \"{}\",", w.evm_address);
    println!("  \"bitcoin_address\": \"{}\",", w.bitcoin_address);
    println!("  \"cosmos_address\": \"{}\",", w.cosmos_address);
    println!("  \"polkadot_address\": \"{}\",", w.polkadot_address);
    println!("  \"xaddress\": \"{}\",", styled_xaddress);
    println!("  \"xidentity\": \"{}\",", w.xidentity);
    println!("  \"collapsed_at\": {}", now);
    println!("}}");
}

/// -------- ABI call helper --------

unsafe fn call_and_parse<T: serde::de::DeserializeOwned>(
    acegf_call: &Symbol<unsafe extern "C" fn(*const ACEGF_Call, *mut ACEGF_Result) -> c_int>,
    acegf_free_result: &Symbol<unsafe extern "C" fn(*mut ACEGF_Result)>,
    call: &ACEGF_Call,
) -> Result<T, String> {
    let mut out = ACEGF_Result {
        code: 0,
        data: std::ptr::null(),
        data_len: 0,
        reserved: [0; 8],
    };

    let rc = acegf_call(call as *const _, &mut out as *mut _);
    if rc != 0 || out.data.is_null() {
        acegf_free_result(&mut out);
        return Err(format!("acegf_call failed (rc={}, code={})", rc, out.code));
    }

    let json = CStr::from_ptr(out.data).to_string_lossy().into_owned();
    acegf_free_result(&mut out);

    serde_json::from_str(&json)
        .map_err(|e| format!("JSON parse error: {} | raw={}", e, json))
}

/// -------- main --------

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  {} generate <passphrase>", args[0]);
        eprintln!("  {} rekey <old_pass> <new_pass> <mnemonic>", args[0]);
        eprintln!("  {} view <passphrase> <mnemonic>", args[0]);
        eprintln!("  {} aceize <passphrase> <legacy mnemonic>", args[0]);
        process::exit(1);
    }

    let lib_path = resolve_library_path();
    if !lib_path.exists() {
        eprintln!("ACEGF library not found: {:?}", lib_path);
        process::exit(1);
    }

    unsafe {
        let lib = Library::new(lib_path).expect("Failed to load libacegf");

        let acegf_call: Symbol<
            unsafe extern "C" fn(*const ACEGF_Call, *mut ACEGF_Result) -> c_int
        > = lib.get(b"acegf_call").expect("missing acegf_call");

        let acegf_free_result: Symbol<
            unsafe extern "C" fn(*mut ACEGF_Result)
        > = lib.get(b"acegf_free_result").expect("missing acegf_free_result");

        match args[1].as_str() {

            "generate" => {
                if args.len() != 3 {
                    eprintln!("Usage: {} generate <passphrase>", args[0]);
                    process::exit(1);
                }

                let pass = CString::new(args[2].as_str()).unwrap();
                let call = ACEGF_Call {
                    method: ACEGF_METHOD_GENERATE,
                    input_1: pass.as_ptr(),
                    input_2: std::ptr::null(),
                    input_3: std::ptr::null(),
                };

                match call_and_parse::<CryptoEntity>(&acegf_call, &acegf_free_result, &call) {
                    Ok(entity) => print_wallet(&entity),
                    Err(e) => eprintln!("Generate failed: {}", e),
                }
            }

            // -------- rekey --------
            "rekey" => {
                if args.len() != 5 {
                    eprintln!("Usage: {} rekey <old_pass> <new_pass> <mnemonic>", args[0]);
                    process::exit(1);
                }

                let mnemonic = CString::new(args[4].as_str()).unwrap();
                let old = CString::new(args[2].as_str()).unwrap();
                let newp = CString::new(args[3].as_str()).unwrap();

                let call = ACEGF_Call {
                    method: ACEGF_METHOD_REKEY,
                    input_1: mnemonic.as_ptr(),
                    input_2: old.as_ptr(),
                    input_3: newp.as_ptr(),
                };

                let v: serde_json::Value =
                    call_and_parse(&acegf_call, &acegf_free_result, &call)
                        .unwrap_or_else(|e| {
                            eprintln!("Rekey failed: {}", e);
                            process::exit(1);
                        });

                println!("New mnemonic:\n{}", v["mnemonic"]);
            }

            // -------- view --------
            "view" => {
                if args.len() != 4 {
                    eprintln!("Usage: {} view <passphrase> <mnemonic>", args[0]);
                    process::exit(1);
                }

                let pass = CString::new(args[2].as_str()).unwrap();
                let mnemonic = CString::new(args[3].as_str()).unwrap();

                let call = ACEGF_Call {
                    method: ACEGF_METHOD_VIEW,
                    input_1: mnemonic.as_ptr(),
                    input_2: pass.as_ptr(),
                    input_3: std::ptr::null(),
                };

                match call_and_parse::<CryptoEntity>(&acegf_call, &acegf_free_result, &call) {
                    Ok(entity) => print_wallet(&entity),
                    Err(e) => eprintln!("View failed: {}", e),
                }
            }

            // -------- aceize --------
            "aceize" => {
                if args.len() != 4 {
                    eprintln!("Usage: {} aceize <passphrase> <legacy mnemonic>", args[0]);
                    process::exit(1);
                }

                let pass = CString::new(args[2].as_str()).unwrap();
                let mnemonic = CString::new(args[3].as_str()).unwrap();

                let call = ACEGF_Call {
                    method: ACEGF_METHOD_WEITOA,
                    input_1: mnemonic.as_ptr(),
                    input_2: pass.as_ptr(),
                    input_3: std::ptr::null(),
                };

                match call_and_parse::<CryptoEntity>(&acegf_call, &acegf_free_result, &call) {
                    Ok(entity) => print_wallet(&entity),
                    Err(e) => eprintln!("Aceize failed: {}", e),
                }
            }
            "version" => {
                // 1️⃣ load symbol
                let acegf_version: Symbol<
                    unsafe extern "C" fn() -> *mut c_char
                > = lib.get(b"acegf_version")
                    .expect("missing acegf_version");

                // 2️⃣ call
                let ptr = unsafe { acegf_version() };
                if ptr.is_null() {
                    eprintln!("acegf_version returned null");
                    process::exit(1);
                }

                // 3️⃣ convert to Rust string
                let version = unsafe {
                    CStr::from_ptr(ptr).to_string_lossy().into_owned()
                };

                // 4️⃣ free memory (paired with to_cstring / into_raw)
                unsafe {
                    // IMPORTANT: must match allocation side
                    let _ = CString::from_raw(ptr);
                }

                // 5️⃣ print
                println!("ACE-GF Version: {}", version);
            }

            _ => {
                eprintln!("Unknown command: {}", args[1]);
                process::exit(1);
            }
        }
    }
}
