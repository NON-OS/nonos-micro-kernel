use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=NONOS_SIGNING_KEY");
    println!("cargo:rerun-if-env-changed=NONOS_MLDSA65_PUBKEY");
    println!("cargo:rerun-if-env-changed=NONOS_TRUST_ANCHOR_PUBKEY");
    println!("cargo:rerun-if-env-changed=NONOS_ZK_DEVICE_ROOT");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rerun-if-changed=boot-splash.png");

    generate_keys();
    generate_zk_registry();
    configure_uefi();
    configure_optimization();
    configure_crypto();
    compile_mldsa65();
    configure_security();
    embed_build_info();
}

// Reproducible build timestamp. SOURCE_DATE_EPOCH is the de-facto
// standard env var for reproducible builds; the Makefile pins it
// to the latest git commit time. The SystemTime fallback only
// fires for ad-hoc builds outside the make pipeline; production
// builds set SOURCE_DATE_EPOCH explicitly.
fn build_timestamp_secs() -> u64 {
    if let Ok(s) = env::var("SOURCE_DATE_EPOCH") {
        if let Ok(v) = s.parse::<u64>() {
            return v;
        }
    }
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn cargo_feature(name: &str) -> bool {
    env::var_os(format!("CARGO_FEATURE_{name}")).is_some()
}

fn production_mode() -> bool {
    cargo_feature("PRODUCTION") || cargo_feature("HARDENED_PRODUCTION") || cargo_feature("HARDENED")
}

fn dev_mode() -> bool {
    cargo_feature("DEV_QEMU") || cargo_feature("DEV_MODE")
}

fn build_mode_name() -> &'static str {
    if production_mode() {
        "production"
    } else if dev_mode() {
        "dev-qemu"
    } else {
        "standard"
    }
}

fn generate_keys() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("keys_generated.rs");

    let public_key = resolve_public_key();
    let mldsa65_key = resolve_mldsa65_public_key();
    let key_id = compute_key_id(&public_key);

    let fingerprint = format!(
        "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        key_id[0], key_id[1], key_id[2], key_id[3], key_id[4], key_id[5], key_id[6], key_id[7]
    );

    let build_timestamp = build_timestamp_secs();

    let mut file = fs::File::create(&dest_path).expect("Cannot create keys_generated.rs");

    writeln!(file, "pub const NONOS_PUBLIC_KEY: [u8; 32] = [").unwrap();
    write!(file, "    ").unwrap();
    for (i, byte) in public_key.iter().enumerate() {
        write!(file, "0x{:02x}", byte).unwrap();
        if i < 31 {
            write!(file, ", ").unwrap();
        }
        if (i + 1) % 8 == 0 && i < 31 {
            writeln!(file).unwrap();
            write!(file, "    ").unwrap();
        }
    }
    writeln!(file, "\n];").unwrap();

    writeln!(file, "#[allow(dead_code)]").unwrap();
    writeln!(file, "pub const NONOS_MLDSA65_PUBLIC_KEY: [u8; 1952] = [").unwrap();
    write!(file, "    ").unwrap();
    for (i, byte) in mldsa65_key.iter().enumerate() {
        write!(file, "0x{:02x}", byte).unwrap();
        if i < 1951 {
            write!(file, ", ").unwrap();
        }
        if (i + 1) % 8 == 0 && i < 1951 {
            writeln!(file).unwrap();
            write!(file, "    ").unwrap();
        }
    }
    writeln!(file, "\n];").unwrap();

    writeln!(file, "pub const NONOS_KEY_ID: [u8; 32] = [").unwrap();
    write!(file, "    ").unwrap();
    for (i, byte) in key_id.iter().enumerate() {
        write!(file, "0x{:02x}", byte).unwrap();
        if i < 31 {
            write!(file, ", ").unwrap();
        }
        if (i + 1) % 8 == 0 && i < 31 {
            writeln!(file).unwrap();
            write!(file, "    ").unwrap();
        }
    }
    writeln!(file, "\n];").unwrap();

    writeln!(file, "pub const KEY_FINGERPRINT: &str = \"{}\";", fingerprint).unwrap();
    writeln!(file, "pub const BUILD_TIMESTAMP: u64 = {};", build_timestamp).unwrap();
    writeln!(file, "pub const KEY_VERSION: u32 = 1;").unwrap();

    println!("cargo:rustc-env=NONOS_KEY_FINGERPRINT={}", fingerprint);
    eprintln!("NONOS key fingerprint: {}", fingerprint);
}

fn resolve_public_key() -> [u8; 32] {
    if let Ok(path) = env::var("NONOS_TRUST_ANCHOR_PUBKEY") {
        if !path.trim().is_empty() {
            println!("cargo:rerun-if-changed={}", path);
            let data = fs::read(&path).unwrap_or_else(|e| {
                panic!("FATAL: cannot read trust anchor public key at {}: {}", path, e)
            });
            if data.len() != 32 {
                panic!("trust anchor public key must be 32 bytes, got {}", data.len());
            }
            return data[..32].try_into().expect("pubkey length");
        }
    }
    let signing_key_path = resolve_signing_key_path();
    println!("cargo:rerun-if-changed={}", signing_key_path);
    let key_data = fs::read(&signing_key_path).unwrap_or_else(|e| {
        panic!("FATAL: Cannot read signing key at {}: {}", signing_key_path, e)
    });
    if key_data.len() < 32 {
        panic!("Signing key must be at least 32 bytes, got {}", key_data.len());
    }
    let seed: [u8; 32] = key_data[..32].try_into().expect("seed length");
    derive_ed25519_public_key(&seed)
}

fn resolve_mldsa65_public_key() -> [u8; 1952] {
    let path = match env::var("NONOS_MLDSA65_PUBKEY") {
        Ok(path) if !path.trim().is_empty() => path,
        _ if production_mode() => panic!("production bootloader requires NONOS_MLDSA65_PUBKEY"),
        _ => String::from(".keys/kernel_mldsa65.pub"),
    };
    println!("cargo:rerun-if-changed={}", path);
    let data = fs::read(&path).unwrap_or_else(|e| {
        panic!("cannot read ML-DSA-65 public key at {}: {}", path, e)
    });
    parse_mldsa65_public_key(&data)
}

fn parse_mldsa65_public_key(data: &[u8]) -> [u8; 1952] {
    if data.len() != 1963 || &data[..8] != b"NONOSPK1" {
        panic!("ML-DSA-65 public key file has invalid envelope");
    }
    if data[8] != 0x03 {
        panic!("ML-DSA-65 public key file has wrong algorithm");
    }
    let len = u16::from_be_bytes([data[9], data[10]]) as usize;
    if len != 1952 {
        panic!("ML-DSA-65 public key length must be 1952 bytes");
    }
    data[11..1963].try_into().expect("mldsa65 pubkey length")
}

fn resolve_signing_key_path() -> String {
    match env::var("NONOS_SIGNING_KEY") {
        Ok(path) if !path.trim().is_empty() => {
            if production_mode() && !Path::new(&path).exists() {
                panic!("production bootloader requires NONOS_SIGNING_KEY to point at an existing release key");
            }
            path
        }
        _ if production_mode() => {
            panic!("production bootloader requires NONOS_SIGNING_KEY and must not generate a signing key");
        }
        _ => resolve_development_signing_key(),
    }
}

fn resolve_development_signing_key() -> String {
    let default = "keys/signing_key_v1.bin";
    if Path::new(default).exists() {
        return String::from(default);
    }
    eprintln!("NOTE: No signing key found. Generating development key...");
    eprintln!("      For production, set NONOS_SIGNING_KEY to an external release key");
    let key_dir = Path::new("keys");
    if !key_dir.exists() {
        fs::create_dir_all(key_dir).expect("Cannot create keys directory");
    }
    let key = generate_development_signing_key();
    fs::write(default, &key).expect("Cannot write development key");
    eprintln!("      Development key written to {}", default);
    String::from(default)
}

fn generate_development_signing_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    if let Ok(mut f) = fs::File::open("/dev/urandom") {
        use std::io::Read;
        if f.read_exact(&mut key).is_ok() {
            return key;
        }
    }
    // Deterministic fallback only if /dev/urandom is unavailable.
    // Production environments expose /dev/urandom; this path keeps
    // dev builds reproducible against SOURCE_DATE_EPOCH.
    let seed = build_timestamp_secs() as u128;
    for (i, byte) in key.iter_mut().enumerate() {
        *byte = ((seed >> (i % 16 * 8)) & 0xFF) as u8 ^ (i as u8).wrapping_mul(17);
    }
    key
}

fn derive_ed25519_public_key(seed: &[u8; 32]) -> [u8; 32] {
    compute_ed25519_pubkey(seed)
}

fn generate_zk_registry() {
    generate_transparent_zk_registry();
}

fn generate_transparent_zk_registry() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("zk_generated.rs");
    fs::write(&dest_path, b"").expect("Cannot create zk_generated.rs");
    let root_path = resolve_device_root_path();
    let root = fs::read(&root_path).unwrap_or_else(|e| {
        panic!("production bootloader requires NONOS_ZK_DEVICE_ROOT to be readable: {e}")
    });
    if root.len() != 32 {
        panic!("production bootloader requires 32-byte NONOS_ZK_DEVICE_ROOT");
    }
    println!("cargo:rerun-if-changed={root_path}");
    println!("cargo:rustc-env=NONOS_ZK_DEVICE_ROOT={root_path}");
    let fp = blake3::hash(&root);
    let fp_hex = fp.as_bytes().iter().take(8).map(|b| format!("{:02x}", b)).collect::<String>();
    println!("cargo:rustc-env=NONOS_ZK_FINGERPRINT={fp_hex}");
    eprintln!("NONOS transparent ZK device root fingerprint: {fp_hex}");
}

fn resolve_device_root_path() -> String {
    match env::var("NONOS_ZK_DEVICE_ROOT") {
        Ok(path) if !path.trim().is_empty() => path,
        _ => panic!("production bootloader requires NONOS_ZK_DEVICE_ROOT"),
    }
}

fn compute_ed25519_pubkey(scalar: &[u8; 32]) -> [u8; 32] {
    use ed25519_dalek::SigningKey;
    let signing_key = SigningKey::from_bytes(scalar);
    signing_key.verifying_key().to_bytes()
}

fn compute_key_id(public_key: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:KEYID:ED25519:v1");
    hasher.update(public_key);
    *hasher.finalize().as_bytes()
}

fn configure_uefi() {
    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("uefi") {
        return;
    }
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-zmax-page-size=0x1000");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=--gc-sections");
    println!("cargo:rustc-link-arg=/SUBSYSTEM:EFI_APPLICATION");
    println!("cargo:rustc-link-arg=/ENTRY:efi_main");
    println!("cargo:rustc-link-arg=/MERGE:.rdata=.data");
}

fn configure_optimization() {
    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("uefi") {
        return;
    }
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    if profile == "release" {
        println!("cargo:rustc-env=CARGO_CFG_LTO=fat");
        println!("cargo:rustc-link-arg=-Os");
        println!("cargo:rustc-link-arg=--strip-all");
        println!("cargo:rustc-link-arg=-z,relro");
        println!("cargo:rustc-link-arg=-z,now");
        println!("cargo:rustc-link-arg=-z,noexecstack");
    }
}

fn configure_crypto() {
    if cfg!(feature = "efi-rng") {
        println!("cargo:rustc-cfg=feature=\"hardware_rng\"");
        println!("cargo:rustc-env=NONOS_HW_RNG=1");
    }
    println!("cargo:rustc-cfg=blake3_no_sse2");
    println!("cargo:rustc-cfg=blake3_no_sse41");
    println!("cargo:rustc-cfg=blake3_no_avx2");
    println!("cargo:rustc-cfg=blake3_no_avx512");
}

fn compile_mldsa65() {
    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("uefi") {
        return;
    }
    let mldsa = "../third_party/pqclean/crypto_sign/ml-dsa-65/clean";
    let common = "../third_party/pqclean/common";
    let mut build = cc::Build::new();
    build.compiler("clang");
    build.flag("-target");
    build.flag("x86_64-unknown-windows");
    for entry in fs::read_dir(mldsa).expect("cannot read ML-DSA-65 source directory") {
        let path = entry.expect("bad ML-DSA-65 source entry").path();
        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            build.file(path);
        }
    }
    build.file(format!("{common}/fips202.c"));
    build.file("src/crypto/mldsa65/chkstk.c");
    build.file("src/crypto/mldsa65/uefi_alloc.c");
    build.file("src/crypto/mldsa65/randombytes.c");
    build.file("src/crypto/mldsa65/mem.c");
    build.include("src/crypto/mldsa65/pqclean_shim");
    build.include(mldsa);
    build.include(common);
    build.flag_if_supported("-ffreestanding");
    build.flag_if_supported("-fno-stack-protector");
    build.flag_if_supported("-mno-stack-arg-probe");
    build.flag_if_supported("-ffunction-sections");
    build.flag_if_supported("-fdata-sections");
    build.compile("nonos_boot_mldsa65");
}

fn configure_security() {
    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("uefi") {
        return;
    }
    if cfg!(feature = "nonos-cet") {
        println!("cargo:rustc-link-arg=-fcf-protection=full");
        println!("cargo:rustc-env=NONOS_CET_ENABLED=1");
    }
    println!("cargo:rustc-link-arg=-fstack-protector-strong");
    println!("cargo:rustc-link-arg=-fpie");
    println!("cargo:rustc-link-arg=/DYNAMICBASE");
    println!("cargo:rustc-link-arg=/HIGHENTROPYVA");
    println!("cargo:rustc-link-arg=/NXCOMPAT");
}

fn embed_build_info() {
    let build_time = format!("{}", build_timestamp_secs());

    println!("cargo:rustc-env=NONOS_BUILD_TIME={build_time}");

    if let Ok(output) =
        std::process::Command::new("git").args(["rev-parse", "--short", "HEAD"]).output()
    {
        let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("cargo:rustc-env=NONOS_GIT_COMMIT={commit}");
    } else {
        println!("cargo:rustc-env=NONOS_GIT_COMMIT=unknown");
    }

    let rustc_version = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=NONOS_RUSTC_VERSION={rustc_version}");
    println!("cargo:rustc-env=NONOS_BOOTLOADER_NAME=NONOS Bootloader");
    println!("cargo:rustc-env=NONOS_BOOTLOADER_VERSION=1.0.0");
    println!("cargo:rustc-env=NONOS_BOOT_BUILD_MODE={}", build_mode_name());
}
