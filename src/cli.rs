use libloading::{Library, Symbol};
use std::env;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::path::PathBuf;
use std::process;

#[repr(C)]
pub struct GeneratedWallet {
    pub mnemonic: *mut c_char,
    pub solana_address: *mut c_char,
    pub evm_address: *mut c_char,
    pub bitcoin_address: *mut c_char,
    pub cosmos_address: *mut c_char,
    pub polkadot_address: *mut c_char,
    pub xidentity: *mut c_char,
}

/// Locate the correct dynamic library based on OS
fn resolve_library_path() -> PathBuf {
    // CLI binary path (e.g. playground/cli/mscikdf_cli_linux)
    let exe = env::current_exe().expect("Cannot get current executable path");
    let cli_dir = exe.parent().expect("Executable has no parent directory");

    // Playground root: one level above cli/
    let root = cli_dir.parent().expect("CLI dir has no parent");

    #[cfg(target_os = "linux")]
    let lib_path = root.join("lib/linux-x86_64/libmscikdf.so");

    #[cfg(target_os = "macos")]
    let lib_path = root.join("lib/macos-x86_64/libmscikdf.dylib");

    if !lib_path.exists() {
        eprintln!("ERROR: Cannot find MSCIKDF library at: {:?}", lib_path);
        eprintln!("Make sure your directory looks like:");
        eprintln!("  mscikdf-playground/");
        eprintln!("    cli/mscikdf_cli_<platform>");
        eprintln!("    lib/<platform>/libmscikdf.so|.dylib");
        process::exit(1);
    }

    lib_path
}

#[allow(mismatched_lifetime_syntaxes)]
unsafe fn load_symbols(lib: &Library) -> (
    Symbol<extern "C" fn(*const c_char) -> GeneratedWallet>,
    Symbol<extern "C" fn(*const c_char, *const c_char, *const c_char) -> *mut c_char>,
    Symbol<extern "C" fn(GeneratedWallet)>,
    Symbol<extern "C" fn() -> *mut c_char>,
    Symbol<extern "C" fn(*const c_char, *const c_char) -> GeneratedWallet>, // 新增
    Symbol<extern "C" fn(*const c_char, *const c_char) -> GeneratedWallet>, // 新增
) {
    let gen: Symbol<extern "C" fn(*const c_char) -> GeneratedWallet> =
        lib.get(b"mscikdf_generate").expect("missing symbol: mscikdf_generate");

    let rekey: Symbol<extern "C" fn(*const c_char, *const c_char, *const c_char) -> *mut c_char> =
        lib.get(b"mscikdf_change_mnemonic_passphrase").expect("missing symbol: mscikdf_change_mnemonic_passphrase");

    let free_wallet: Symbol<extern "C" fn(GeneratedWallet)> =
        lib.get(b"mscikdf_free_wallet").expect("missing symbol: mscikdf_free_wallet");

    let version: Symbol<extern "C" fn() -> *mut c_char> =
        lib.get(b"mscikdf_version").expect("missing symbol: mscikdf_version");

    let restore: Symbol<extern "C" fn(*const c_char, *const c_char) -> GeneratedWallet> =
        lib.get(b"mscikdf_view_wallet").expect("missing symbol: mscikdf_view_wallet");

    let export: Symbol<extern "C" fn(*const c_char, *const c_char) -> GeneratedWallet> =
        lib.get(b"mscikdf_export_private_keys").expect("missing symbol: mscikdf_export_private_keys");

    (gen, rekey, free_wallet, version, restore, export)
}

fn print_wallet(w: &GeneratedWallet) {
    macro_rules! field {
        ($f:ident) => {
            if w.$f.is_null() {
                "ERROR".to_string()
            } else {
                unsafe { CStr::from_ptr(w.$f).to_string_lossy().into_owned() }
            }
        };
    }

    println!(
        "mnemonic : {}\nsolana   : {}\nevm      : {}\nbitcoin  : {}\ncosmos   : {}\npolkadot : {}\nxidentity: {}",
        field!(mnemonic),
        field!(solana_address),
        field!(evm_address),
        field!(bitcoin_address),
        field!(cosmos_address),
        field!(polkadot_address),
        field!(xidentity),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <version/generate/rekey/restore/export> <passphrase> [<mnemonic word list>]", args[0]);
        process::exit(1);
    }

    let lib_path = resolve_library_path();

    unsafe {
        let lib = Library::new(lib_path).expect("Failed to load MSCIKDF library");

        let (mscikdf_generate,
             mscikdf_change_mnemonic_passphrase,
             free_wallet,
             mscikdf_version,
             mscikdf_restore_from_mnemonic,
             mscikdf_export_private_keys) = load_symbols(&lib);

        match args[1].as_str() {
            "generate" => {
                if args.len() != 3 {
                 // 期望: 程序名 + generate + pass => 共 3 个参数
                    eprintln!("Usage: {} generate <pass>", args[0]);
                    process::exit(1);
                }
                let pass = CString::new(args[2].as_str()).unwrap();
                let wallet = mscikdf_generate(pass.as_ptr());
                print_wallet(&wallet);
                free_wallet(wallet);
            }

            "rekey" => {
                if args.len() != 28 {
                    // 期望: 程序名 + rekey + old_pass + new_pass + 24个单词 => 共 28 个参数
                    eprintln!("Usage: {} rekey <old_pass> <new_pass> <24-word mnemonic>", args[0]);
                    process::exit(1);
                }

                let old = CString::new(args[2].as_str()).unwrap();

                let newp = CString::new(args[3].as_str()).unwrap();

                let words = &args[4..];
                let mnemonic_str = words.join(" ");
                let mnemonic_c = CString::new(mnemonic_str).unwrap();

                let new_mn = mscikdf_change_mnemonic_passphrase(
                    mnemonic_c.as_ptr(),
                    old.as_ptr(),
                    newp.as_ptr(),
                );

                if new_mn.is_null() {
                    eprintln!("Rekey failed");
                    process::exit(1);
                }

                let s = CStr::from_ptr(new_mn).to_string_lossy().into_owned();
                println!("New mnemonic: {}", s);
            }
            "restore" => {
                 // 期望: 程序名 + restore + passphrase + 24个单词 => 共 27 个参数
                if args.len() != 27 {
                    eprintln!("Usage: {} restore <passphrase> <word1> <word2> ... <word24>", args[0]);
                    eprintln!("   Must provide exactly 24 mnemonic words.");
                    process::exit(1);
                }

                let passphrase = CString::new(args[2].as_str()).expect("Invalid passphrase");
                let mnemonic_words = &args[3..]; // 24 个单词
                let mnemonic = mnemonic_words.join(" ");
                let mnemonic_c = CString::new(mnemonic.as_str()).expect("Invalid mnemonic word");

                let wallet = mscikdf_restore_from_mnemonic(mnemonic_c.as_ptr(), passphrase.as_ptr());

                // 检查是否恢复失败（假设返回的 wallet 所有指针为 null 表示失败）
                if wallet.mnemonic.is_null() && wallet.solana_address.is_null() {
                    eprintln!("Restore failed: invalid mnemonic or passphrase");
                    process::exit(1);
                }

                print_wallet(&wallet);
                free_wallet(wallet);
            }

            "export" => {
                // 期望: 程序名 + restore + passphrase + 24个单词 => 共 26 个参数
                if args.len() != 27 {
                    eprintln!("Usage: {} export <passphrase> <word1> <word2> ... <word24>", args[0]);
                    eprintln!("   Must provide exactly 24 mnemonic words.");
                    process::exit(1);
                }

                let passphrase = CString::new(args[2].as_str()).expect("Invalid passphrase");
                let mnemonic_words = &args[3..]; // 24 个单词
                let mnemonic = mnemonic_words.join(" ");
                let mnemonic_c = CString::new(mnemonic.as_str()).expect("Invalid mnemonic word");

                let wallet = mscikdf_export_private_keys(mnemonic_c.as_ptr(), passphrase.as_ptr());

                // 检查是否恢复失败（假设返回的 wallet 所有指针为 null 表示失败）
                if wallet.mnemonic.is_null() && wallet.solana_address.is_null() {
                    eprintln!("Export failed: invalid mnemonic or passphrase");
                    process::exit(1);
                }

                print_wallet(&wallet);
                free_wallet(wallet);
            }

            "version" => {
                let ver_ptr = mscikdf_version();
                if ver_ptr.is_null() {
                    eprintln!("Failed to get version");
                    process::exit(1);
                }
                let v = CStr::from_ptr(ver_ptr).to_string_lossy().into_owned();
                println!("MSCIKDF Version: {}", v);
            }

            _ => {
                eprintln!("Invalid command");
                process::exit(1);
            }
        }
    }
}
