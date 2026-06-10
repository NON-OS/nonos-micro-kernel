use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=NONOS_SIGNING_KEY");
    println!("cargo:rerun-if-env-changed=NONOS_ZK_CEREMONY_DIR");
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rerun-if-changed=boot-splash.png");
    println!("cargo:rerun-if-changed=zk/ceremony/");

    generate_keys();
    generate_zk_registry();
    generate_background_image();
    configure_uefi();
    configure_optimization();
    configure_crypto();
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

    let signing_key_path = resolve_signing_key_path();

    println!("cargo:rerun-if-changed={}", signing_key_path);

    let key_data = fs::read(&signing_key_path).unwrap_or_else(|e| {
        panic!(
            "FATAL: Cannot read signing key at {}: {}\n\
             Generate a key with: ./tools/keygen/keygen.py --output {}",
            signing_key_path, e, signing_key_path
        )
    });

    if key_data.len() < 32 {
        panic!("Signing key must be at least 32 bytes, got {}", key_data.len());
    }

    let seed: [u8; 32] = key_data[..32].try_into().expect("seed length");
    let public_key = derive_ed25519_public_key(&seed);
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
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("zk_generated.rs");

    let ceremony_dir = resolve_ceremony_dir();

    let circuits = [
        ("attestation-program", "zkmod-attestation-program-v1", "attestation_program"),
        ("boot-authority", "zkmod-boot-authority-v1", "boot_authority"),
        ("update-authority", "zkmod-update-authority-v1", "update_authority"),
        ("recovery-key", "zkmod-recovery-key-v1", "recovery_key"),
    ];

    let mut program_hashes: Vec<(String, [u8; 32])> = Vec::new();
    let mut vk_data: Vec<(String, Vec<u8>)> = Vec::new();
    let mut vk_fingerprints: Vec<(String, [u8; 32])> = Vec::new();

    for (name, program_id, const_name) in &circuits {
        let program_hash = compute_program_hash(program_id);
        program_hashes.push((const_name.to_string(), program_hash));

        let vk_bytes = load_or_generate_vk(&ceremony_dir, name);
        let vk_fp = compute_vk_fingerprint(&vk_bytes);

        vk_data.push((const_name.to_string(), vk_bytes));
        vk_fingerprints.push((const_name.to_string(), vk_fp));
    }

    let mut file = fs::File::create(&dest_path).expect("Cannot create zk_generated.rs");
    use std::io::Write;

    for (const_name, hash) in &program_hashes {
        writeln!(file, "pub const PROGRAM_HASH_{}: [u8; 32] = [", const_name.to_uppercase())
            .unwrap();
        write_byte_array(&mut file, hash);
        writeln!(file, "];").unwrap();
    }

    for (const_name, fp) in &vk_fingerprints {
        writeln!(file, "pub const VK_FINGERPRINT_{}: [u8; 32] = [", const_name.to_uppercase())
            .unwrap();
        write_byte_array(&mut file, fp);
        writeln!(file, "];").unwrap();
    }

    let vk_bin_path = Path::new(&out_dir).join("vk_all.bin");
    let mut offsets: Vec<(String, usize, usize)> = Vec::new();
    let mut all_vk_bytes: Vec<u8> = Vec::new();

    for (const_name, vk) in &vk_data {
        let offset = all_vk_bytes.len();
        all_vk_bytes.extend_from_slice(vk);
        offsets.push((const_name.clone(), offset, vk.len()));
    }

    fs::write(&vk_bin_path, &all_vk_bytes).expect("Cannot write vk_all.bin");

    writeln!(file, "pub const VK_ALL_BYTES: &[u8] = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/vk_all.bin\"));").unwrap();

    for (const_name, offset, len) in &offsets {
        writeln!(file, "pub const VK_{}_OFFSET: usize = {};", const_name.to_uppercase(), offset)
            .unwrap();
        writeln!(file, "pub const VK_{}_LEN: usize = {};", const_name.to_uppercase(), len).unwrap();
    }

    writeln!(file, "pub const ZK_REGISTRY_VERSION: u32 = 1;").unwrap();

    let build_timestamp = build_timestamp_secs();
    writeln!(file, "pub const ZK_BUILD_TIMESTAMP: u64 = {};", build_timestamp).unwrap();

    let is_ceremony = !ceremony_dir.is_empty() && Path::new(&ceremony_dir).exists();
    writeln!(file, "pub const ZK_FROM_CEREMONY: bool = {};", is_ceremony).unwrap();

    let fp_overall = compute_overall_fingerprint(&program_hashes, &vk_fingerprints);
    writeln!(file, "pub const ZK_REGISTRY_FINGERPRINT: [u8; 32] = [").unwrap();
    write_byte_array(&mut file, &fp_overall);
    writeln!(file, "];").unwrap();

    let fp_hex = fp_overall.iter().take(8).map(|b| format!("{:02x}", b)).collect::<String>();
    println!("cargo:rustc-env=NONOS_ZK_FINGERPRINT={}", fp_hex);
    eprintln!("NONOS ZK registry fingerprint: {}", fp_hex);
    eprintln!("ZK circuits: {} (ceremony: {})", circuits.len(), is_ceremony);
}

fn compute_program_hash(program_id: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:ZK:PROGRAM:v1");
    hasher.update(program_id.as_bytes());
    *hasher.finalize().as_bytes()
}

fn compute_vk_fingerprint(vk_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:VK:FINGERPRINT:v1");
    hasher.update(vk_bytes);
    *hasher.finalize().as_bytes()
}

fn compute_overall_fingerprint(
    hashes: &[(String, [u8; 32])],
    fps: &[(String, [u8; 32])],
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:ZK:REGISTRY:v1");
    for (_, h) in hashes {
        hasher.update(h);
    }
    for (_, f) in fps {
        hasher.update(f);
    }
    *hasher.finalize().as_bytes()
}

fn load_or_generate_vk(ceremony_dir: &str, circuit_name: &str) -> Vec<u8> {
    if !ceremony_dir.is_empty() {
        let vk_path = format!("{}/vk_{}.bin", ceremony_dir, circuit_name.replace('-', "_"));
        if let Ok(data) = fs::read(&vk_path) {
            if data.len() >= 96 {
                eprintln!("  Loaded VK for {} from ceremony ({} bytes)", circuit_name, data.len());
                return data;
            }
        }
    }

    if production_mode() {
        panic!("production bootloader requires signed ceremony VK for {circuit_name}");
    }

    eprintln!("  Generating development VK for {} (NOT FOR PRODUCTION)", circuit_name);
    generate_development_vk(circuit_name)
}

fn resolve_ceremony_dir() -> String {
    match env::var("NONOS_ZK_CEREMONY_DIR") {
        Ok(path) if !path.trim().is_empty() => {
            if production_mode() && !Path::new(&path).exists() {
                panic!("production bootloader requires NONOS_ZK_CEREMONY_DIR to exist");
            }
            path
        }
        _ if production_mode() => {
            panic!(
                "production bootloader requires NONOS_ZK_CEREMONY_DIR and must not generate VKs"
            );
        }
        _ => {
            let default = "zk/ceremony";
            if Path::new(default).exists() {
                String::from(default)
            } else {
                String::new()
            }
        }
    }
}

fn generate_development_vk(circuit_name: &str) -> Vec<u8> {
    let seed = {
        let mut hasher = blake3::Hasher::new_derive_key("NONOS:DEV:VK:v1");
        hasher.update(circuit_name.as_bytes());
        *hasher.finalize().as_bytes()
    };
    let mut vk = vec![0u8; 872];
    for (i, byte) in vk.iter_mut().enumerate() {
        *byte = seed[i % 32] ^ (i as u8);
    }
    vk[0..8].copy_from_slice(b"NONOSVK\x01");
    let checksum = {
        let mut hasher = blake3::Hasher::new_derive_key("NONOS:VK:CHECKSUM:v1");
        hasher.update(&vk[0..864]);
        hasher.finalize()
    };
    vk[864..872].copy_from_slice(&checksum.as_bytes()[0..8]);
    vk
}

fn write_byte_array(file: &mut fs::File, bytes: &[u8; 32]) {
    use std::io::Write;
    write!(file, "    ").unwrap();
    for (i, byte) in bytes.iter().enumerate() {
        write!(file, "0x{:02x}", byte).unwrap();
        if i < 31 {
            write!(file, ", ").unwrap();
        }
        if (i + 1) % 8 == 0 && i < 31 {
            writeln!(file).unwrap();
            write!(file, "    ").unwrap();
        }
    }
    writeln!(file).unwrap();
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

fn generate_background_image() {
    use std::io::Write as IoWrite;

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("background_generated.rs");

    let wallpaper_path = Path::new("boot-splash.png");

    if !wallpaper_path.exists() {
        eprintln!(
            "NOTE: Wallpaper not found at {:?}, generating gradient fallback",
            wallpaper_path
        );
        generate_gradient_fallback(&dest_path);
        return;
    }

    let img = match image::open(wallpaper_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("NOTE: Cannot load wallpaper: {}, generating gradient fallback", e);
            generate_gradient_fallback(&dest_path);
            return;
        }
    };

    let target_w = 1920u32;
    let target_h = 1080u32;

    let scaled = img.resize_exact(target_w, target_h, image::imageops::FilterType::Lanczos3);
    let rgba = scaled.to_rgba8();

    /* RLE compression for repeated pixels */
    let mut compressed: Vec<u8> = Vec::new();
    let pixels = rgba.as_raw();

    let mut i = 0;
    while i < pixels.len() {
        let r = pixels[i];
        let g = pixels[i + 1];
        let b = pixels[i + 2];
        let a = pixels[i + 3];

        let mut run = 1u8;
        while run < 255 && i + (run as usize * 4) < pixels.len() {
            let ni = i + (run as usize * 4);
            if pixels[ni] == r && pixels[ni + 1] == g && pixels[ni + 2] == b && pixels[ni + 3] == a
            {
                run += 1;
            } else {
                break;
            }
        }

        compressed.push(run);
        compressed.push(b); /* BGR order for framebuffer */
        compressed.push(g);
        compressed.push(r);
        compressed.push(a);

        i += run as usize * 4;
    }

    eprintln!(
        "Background: {}x{}, raw {}KB, compressed {}KB ({:.1}% ratio)",
        target_w,
        target_h,
        (target_w * target_h * 4) / 1024,
        compressed.len() / 1024,
        (compressed.len() as f64 / (target_w * target_h * 4) as f64) * 100.0
    );

    let mut file = fs::File::create(&dest_path).expect("Cannot create background_generated.rs");

    writeln!(file, "pub const BG_WIDTH: u32 = {};", target_w).unwrap();
    writeln!(file, "pub const BG_HEIGHT: u32 = {};", target_h).unwrap();
    writeln!(file, "pub const BG_COMPRESSED_LEN: usize = {};", compressed.len()).unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "#[allow(clippy::all)]").unwrap();
    writeln!(file, "pub static BG_COMPRESSED: [u8; {}] = [", compressed.len()).unwrap();

    for (idx, chunk) in compressed.chunks(16).enumerate() {
        write!(file, "    ").unwrap();
        for (i, byte) in chunk.iter().enumerate() {
            write!(file, "0x{:02x}", byte).unwrap();
            if idx * 16 + i < compressed.len() - 1 {
                write!(file, ",").unwrap();
            }
            if i < chunk.len() - 1 {
                write!(file, " ").unwrap();
            }
        }
        writeln!(file, "").unwrap();
    }
    writeln!(file, "];").unwrap();
}

fn generate_gradient_fallback(dest_path: &Path) {
    use std::io::Write as IoWrite;

    /*
     * Generate a nature-inspired gradient as fallback
     * Deep green to soft cyan, like early morning meadow
     */
    let w = 64u32;
    let h = 36u32;

    let mut compressed: Vec<u8> = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 / w as f32;
            let fy = y as f32 / h as f32;

            /* Diagonal gradient: deep forest green to soft teal */
            let t = (fx + fy) / 2.0;
            let r = (10.0 + t * 30.0) as u8;
            let g = (30.0 + t * 60.0) as u8;
            let b = (20.0 + t * 50.0) as u8;

            compressed.push(1); /* run length 1 */
            compressed.push(b);
            compressed.push(g);
            compressed.push(r);
            compressed.push(255);
        }
    }

    let mut file = fs::File::create(dest_path).expect("Cannot create background_generated.rs");

    writeln!(file, "pub const BG_WIDTH: u32 = {};", w).unwrap();
    writeln!(file, "pub const BG_HEIGHT: u32 = {};", h).unwrap();
    writeln!(file, "pub const BG_COMPRESSED_LEN: usize = {};", compressed.len()).unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "#[allow(clippy::all)]").unwrap();
    writeln!(file, "pub static BG_COMPRESSED: [u8; {}] = [", compressed.len()).unwrap();

    for (idx, chunk) in compressed.chunks(16).enumerate() {
        write!(file, "    ").unwrap();
        for (i, byte) in chunk.iter().enumerate() {
            write!(file, "0x{:02x}", byte).unwrap();
            if idx * 16 + i < compressed.len() - 1 {
                write!(file, ",").unwrap();
            }
        }
        writeln!(file, "").unwrap();
    }
    writeln!(file, "];").unwrap();
}
