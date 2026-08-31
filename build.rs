// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=abi/manifest.toml");
    println!("cargo:rerun-if-changed=third_party/pqclean");
    println!("cargo:rerun-if-changed=src/crypto/pqclean_support");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=linker_aarch64.ld");
    println!("cargo:rerun-if-changed=linker_riscv64.ld");
    println!("cargo:rerun-if-changed=nonos-data/trust/capsules");
    println!("cargo:rerun-if-changed=nonos-data/trust/keys");
    println!("cargo:rerun-if-changed=nonos-data/trust/policy");
    rerun_on_capsule_binaries();

    compile_pqclean_mlkem();
    compile_pqclean_mldsa();
    compile_arch_asm();
    configure_kernel_target();
    stage_image_capability_ceiling();
    generate_manifest_and_signature();
    embed_kernel_build_info();
}

// The image capability ceiling is baked into the kernel, but most images do not
// set one: the default is unrestricted and lives nowhere on disk. include_bytes!
// cannot express an optional file, so the source path is staged through OUT_DIR.
// A ceiling file present under the trust policy is copied verbatim; when it is
// absent the eight zero bytes written here decode, per image_ceiling::ceiling,
// to the unrestricted default. This keeps a plain checkout building while still
// letting an image lower its own ceiling by dropping the file.
fn stage_image_capability_ceiling() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src = PathBuf::from("nonos-data/trust/policy/image_capability_ceiling.bin");
    println!("cargo:rerun-if-changed={}", src.display());
    let dst = PathBuf::from(&out_dir).join("image_capability_ceiling.bin");
    let bytes = fs::read(&src).unwrap_or_else(|_| vec![0u8; 8]);
    fs::write(&dst, bytes).expect("stage image capability ceiling");
}

// Assemble src/arch/<arch>/asm/*.S for the kernel target.
fn compile_arch_asm() {
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("apple") || target.contains("linux-gnu") || target.contains("windows") {
        return;
    }

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let Some((clang_target, arch_flags)) = c_target(&arch) else {
        return;
    };
    let subdir = arch.as_str();

    let dir = PathBuf::from(format!("src/arch/{}/asm", subdir));
    if !dir.exists() {
        return;
    }

    let pattern = dir.join("*.S").to_string_lossy().to_string();
    let files: Vec<_> = glob::glob(&pattern)
        .expect("glob arch asm")
        .filter_map(Result::ok)
        .filter(|p| p.exists())
        .collect();

    if files.is_empty() {
        return;
    }

    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
    }

    let mut build = cc::Build::new();
    build
        .compiler("clang")
        .files(&files)
        .flag("-target")
        .flag(clang_target)
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-fno-stack-protector")
        .warnings(false);

    for flag in arch_flags {
        build.flag(flag);
    }

    configure_cross_archive(&mut build);
    build.compile("nonos_arch_asm");
}

fn configure_cross_archive(build: &mut cc::Build) {
    if let Some(path) = find_build_tool("LLVM_AR", "llvm-ar") {
        build.archiver(path);
    }
    if let Some(path) = find_build_tool("LLVM_RANLIB", "llvm-ranlib") {
        build.ranlib(path);
    }
}

fn find_build_tool(env_key: &str, tool: &str) -> Option<PathBuf> {
    if let Some(path) = env::var_os(env_key).map(PathBuf::from).filter(|p| p.exists()) {
        return Some(path);
    }
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(tool);
            if path.exists() {
                return Some(path);
            }
        }
    }
    for dir in ["/usr/local/opt/llvm/bin", "/opt/homebrew/opt/llvm/bin"] {
        let path = PathBuf::from(dir).join(tool);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn compile_pqclean_mlkem() {
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("apple") || target.contains("linux-gnu") || target.contains("windows") {
        return;
    }

    let (kem_dir, kem_macro) = if env::var("CARGO_FEATURE_MLKEM1024").is_ok() {
        ("ml-kem-1024", "MLKEM1024")
    } else if env::var("CARGO_FEATURE_MLKEM512").is_ok() {
        ("ml-kem-512", "MLKEM512")
    } else {
        ("ml-kem-768", "MLKEM768")
    };

    let base = PathBuf::from(format!("third_party/pqclean/crypto_kem/{}/clean", kem_dir));
    let common = PathBuf::from("third_party/pqclean/common");
    if !base.exists() {
        return;
    }

    let pattern = base.join("*.c").to_string_lossy().to_string();
    let mut files: Vec<_> = glob::glob(&pattern)
        .expect("glob failed")
        .filter_map(Result::ok)
        .filter(|p| p.exists())
        .collect();

    let fips = common.join("fips202.c");
    let randombytes = PathBuf::from("src/crypto/pqclean_support/randombytes.c");
    let libc_glue = PathBuf::from("src/crypto/pqclean_support/libc_glue.c");

    if fips.exists() {
        files.push(fips);
    }
    if randombytes.exists() {
        files.push(randombytes);
    }
    if libc_glue.exists() {
        files.push(libc_glue);
    }

    if files.is_empty() {
        return;
    }

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let Some((c_triple, c_flags)) = c_target(&arch) else {
        return;
    };

    let mut build = cc::Build::new();
    build
        .compiler("clang")
        .files(files)
        .include("src/crypto/pqclean_support")
        .include(&base)
        .include(&common)
        .opt_level(2)
        .pic(false)
        .flag("-target")
        .flag(c_triple)
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-fno-strict-aliasing")
        .flag("-fwrapv")
        .flag("-fno-omit-frame-pointer")
        .flag("-fno-tree-vectorize")
        .flag("-fno-stack-protector")
        .flag("-fno-pic")
        .flag("-w")
        .define(kem_macro, None)
        .warnings(false);

    for flag in c_flags {
        build.flag(flag);
    }

    configure_cross_archive(&mut build);
    build.compile("pqclean_mlkem_clean");
}

fn compile_pqclean_mldsa() {
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("apple") || target.contains("linux-gnu") || target.contains("windows") {
        return;
    }

    let (sign_dir, sign_macro) = if env::var("CARGO_FEATURE_MLDSA5").is_ok() {
        ("ml-dsa-87", "MLDSA87")
    } else if env::var("CARGO_FEATURE_MLDSA2").is_ok() {
        ("ml-dsa-44", "MLDSA44")
    } else {
        ("ml-dsa-65", "MLDSA65")
    };

    let base = PathBuf::from(format!("third_party/pqclean/crypto_sign/{}/clean", sign_dir));
    let common = PathBuf::from("third_party/pqclean/common");
    if !base.exists() {
        return;
    }

    let pattern = base.join("*.c").to_string_lossy().to_string();
    let mut files: Vec<_> = glob::glob(&pattern)
        .expect("glob failed")
        .filter_map(Result::ok)
        .filter(|p| p.exists())
        .collect();

    let fips = common.join("fips202.c");
    let randombytes = PathBuf::from("src/crypto/pqclean_support/randombytes.c");
    let libc_glue = PathBuf::from("src/crypto/pqclean_support/libc_glue.c");

    if fips.exists() {
        files.push(fips);
    }
    if randombytes.exists() {
        files.push(randombytes);
    }
    if libc_glue.exists() {
        files.push(libc_glue);
    }

    if files.is_empty() {
        return;
    }

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let Some((c_triple, c_flags)) = c_target(&arch) else {
        return;
    };

    let mut build = cc::Build::new();
    build
        .compiler("clang")
        .files(files)
        .include("src/crypto/pqclean_support")
        .include(&base)
        .include(&common)
        .opt_level(2)
        .pic(false)
        .flag("-target")
        .flag(c_triple)
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-fno-strict-aliasing")
        .flag("-fwrapv")
        .flag("-fno-omit-frame-pointer")
        .flag("-fno-tree-vectorize")
        .flag("-fno-stack-protector")
        .flag("-fno-pic")
        .flag("-w")
        .define(sign_macro, None)
        .warnings(false);

    for flag in c_flags {
        build.flag(flag);
    }

    configure_cross_archive(&mut build);
    build.compile("pqclean_mldsa_clean");
}

fn configure_kernel_target() {
    let target = env::var("TARGET").unwrap_or_default();
    if target.contains("apple") || target.contains("linux-gnu") || target.contains("windows") {
        return;
    }

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let script_name = match arch.as_str() {
        "x86_64" => "linker.ld",
        "aarch64" => "linker_aarch64.ld",
        "riscv64" => "linker_riscv64.ld",
        _ => return,
    };
    let linker_script = format!("{}/{}", manifest_dir, script_name);
    println!("cargo:rustc-link-arg=--script={}", linker_script);
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=--gc-sections");
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=max-page-size=0x1000");
}

fn generate_manifest_and_signature() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let manifest_content = generate_manifest_content();
    let manifest_data_path = format!("{}/manifest.bin", out_dir);
    fs::write(&manifest_data_path, &manifest_content).expect("Failed to write manifest");

    let profile = env::var("PROFILE").unwrap_or_default();
    let sig = match env::var("NONOS_SIGNING_KEY") {
        Ok(p) => {
            let key_path = PathBuf::from(&p);
            if key_path.exists() {
                sign_manifest_ed25519(&manifest_content, key_path).expect("Ed25519 signing failed")
            } else if profile == "release" {
                panic!("NONOS_SIGNING_KEY file not found at {} (required for release builds)", p);
            } else {
                vec![0u8; 64]
            }
        }
        Err(_) if profile == "release" => {
            panic!("NONOS_SIGNING_KEY not set (required for release builds)");
        }
        Err(_) => {
            vec![0u8; 64]
        }
    };

    let signature_data_path = format!("{}/signature.bin", out_dir);
    fs::write(&signature_data_path, &sig).expect("Failed to write signature");

    generate_manifest_asm(&manifest_content, &sig, &out_dir);
}

fn generate_manifest_content() -> Vec<u8> {
    use std::collections::BTreeMap;

    let mut manifest = BTreeMap::new();

    let module_id = blake3::hash(b"nonos_kernel").as_bytes().to_vec();
    manifest.insert("module_id".to_string(), module_id);
    manifest.insert("entry_symbol".to_string(), b"_start".to_vec());
    manifest.insert("required_caps".to_string(), b"memory,interrupts,syscalls".to_vec());

    let heap_size: u64 = 16 * 1024 * 1024;
    manifest.insert("min_heap_bytes".to_string(), heap_size.to_le_bytes().to_vec());

    let version: u32 = 1;
    manifest.insert("version".to_string(), version.to_le_bytes().to_vec());

    let epoch = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(val) => val.parse::<u64>().unwrap_or(0) * 1_000_000_000,
        Err(_) => {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
        }
    };
    manifest.insert("build_epoch_ns".to_string(), epoch.to_le_bytes().to_vec());

    serialize_manifest(manifest)
}

fn serialize_manifest(manifest: std::collections::BTreeMap<String, Vec<u8>>) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(&(manifest.len() as u32).to_le_bytes());
    for (key, value) in manifest {
        result.extend_from_slice(&(key.len() as u32).to_le_bytes());
        result.extend_from_slice(key.as_bytes());
        result.extend_from_slice(&(value.len() as u32).to_le_bytes());
        result.extend_from_slice(&value);
    }
    result
}

fn sign_manifest_ed25519(data: &[u8], key_path: PathBuf) -> Result<Vec<u8>, String> {
    use ed25519_dalek::{Signature, Signer, SigningKey};
    use sha2::{Digest, Sha512};

    let key_bytes = fs::read(&key_path).map_err(|e| format!("read key: {e}"))?;
    let signing_key = if key_bytes.len() == 32 {
        let seed: [u8; 32] = key_bytes.try_into().map_err(|_| "invalid seed length")?;
        SigningKey::from_bytes(&seed)
    } else if key_bytes.len() == 64 {
        let keypair: [u8; 64] = key_bytes.try_into().map_err(|_| "invalid keypair length")?;
        SigningKey::from_keypair_bytes(&keypair).map_err(|e| format!("keypair: {e}"))?
    } else {
        return Err("NONOS_SIGNING_KEY must be 32-byte seed or 64-byte keypair".into());
    };

    let mut h = Sha512::new();
    h.update(b"NONOS_CAPSULE_V1");
    h.update(data);
    let digest = h.finalize();

    let sig: Signature = signing_key.sign(&digest);
    Ok(sig.to_bytes().to_vec())
}

fn embed_kernel_build_info() {
    let build_time = match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(epoch) => format!("epoch:{}", epoch.trim()),
        Err(_) => "reproducible:none".to_string(),
    };
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    println!("cargo:rustc-env=NONOS_KERNEL_BUILD_TIME={}", build_time);

    if let Ok(output) =
        std::process::Command::new("git").args(["rev-parse", "--short", "HEAD"]).output()
    {
        let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("cargo:rustc-env=NONOS_KERNEL_GIT_COMMIT={}", commit);
    } else {
        println!("cargo:rustc-env=NONOS_KERNEL_GIT_COMMIT=unknown");
    }

    println!("cargo:rustc-env=NONOS_KERNEL_NAME=NONOS Kernel");
    println!("cargo:rustc-env=NONOS_KERNEL_VERSION=0.8.3");
}

fn generate_manifest_asm(manifest_content: &[u8], signature: &[u8], out_dir: &str) {
    let manifest_hex: String = manifest_content.iter().map(|b| format!("0x{:02x}, ", b)).collect();
    let signature_hex: String = signature.iter().map(|b| format!("0x{:02x}, ", b)).collect();

    let asm_content = format!(
        r#".section .nonos.manifest, "a", @progbits
.global __nonos_manifest_data
.global __nonos_manifest_size
__nonos_manifest_data:
    .byte {manifest_hex}
__nonos_manifest_size:
    .quad {manifest_len}

.section .nonos.sig, "a", @progbits
.global __nonos_signature_data
.global __nonos_signature_size
__nonos_signature_data:
    .byte {signature_hex}
__nonos_signature_size:
    .quad {signature_len}
"#,
        manifest_hex = manifest_hex.trim_end_matches(", "),
        manifest_len = manifest_content.len(),
        signature_hex = signature_hex.trim_end_matches(", "),
        signature_len = signature.len()
    );

    let asm_path = format!("{}/manifest_data.s", out_dir);
    fs::write(&asm_path, &asm_content).expect("Failed to write manifest assembly");

    let manifest_bytes: String =
        manifest_content.iter().map(|b| format!("0x{:02x}, ", b)).collect();
    let signature_bytes: String = signature.iter().map(|b| format!("0x{:02x}, ", b)).collect();

    let rs_content = format!(
        r#"pub const MANIFEST_LEN: usize = {manifest_len};
pub const SIGNATURE_LEN: usize = {signature_len};

#[cfg(not(feature = "std"))]
mod _embed {{
    use core::arch::global_asm;

    global_asm!(
        ".section .nonos.manifest, \"aw\", @progbits",
        ".global NONOS_MANIFEST_DATA",
        ".global NONOS_MANIFEST_LEN",
        "NONOS_MANIFEST_DATA:",
        ".byte {manifest_bytes}",
        "NONOS_MANIFEST_LEN:",
        ".quad {manifest_len}",
    );

    global_asm!(
        ".section .nonos.sig, \"aw\", @progbits",
        ".global NONOS_SIGNATURE_DATA",
        ".global NONOS_SIGNATURE_LEN",
        "NONOS_SIGNATURE_DATA:",
        ".byte {signature_bytes}",
        "NONOS_SIGNATURE_LEN:",
        ".quad {signature_len}",
    );
}}

#[cfg(not(feature = "std"))]
extern "C" {{
    pub static NONOS_MANIFEST_DATA: [u8; {manifest_len}];
    pub static NONOS_SIGNATURE_DATA: [u8; {signature_len}];
}}

#[cfg(not(feature = "std"))]
pub fn get_manifest() -> &'static [u8] {{
    unsafe {{ &NONOS_MANIFEST_DATA }}
}}

#[cfg(not(feature = "std"))]
pub fn get_signature() -> &'static [u8] {{
    unsafe {{ &NONOS_SIGNATURE_DATA }}
}}
"#,
        manifest_bytes = manifest_bytes.trim_end_matches(", "),
        manifest_len = manifest_content.len(),
        signature_bytes = signature_bytes.trim_end_matches(", "),
        signature_len = signature.len()
    );

    let rs_path = format!("{}/manifest_data.rs", out_dir);
    fs::write(&rs_path, &rs_content).expect("Failed to write manifest Rust module");
}

// Tell cargo to rebuild the kernel when any embedded capsule ELF on disk
// changes. include_bytes! does not register a dependency on the byte file
// itself, so without this the kernel keeps embedding the previous build.
fn rerun_on_capsule_binaries() {
    let userland = PathBuf::from("userland");
    let Ok(entries) = fs::read_dir(&userland) else {
        return;
    };
    for entry in entries.flatten() {
        let bin_dir = entry.path().join(format!("target/{}/release", user_target())); 
        let Ok(children) = fs::read_dir(&bin_dir) else {
            continue;
        };
        for child in children.flatten() {
            let path = child.path();
            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.contains('.') {
                        println!("cargo:rerun-if-changed={}", path.display());
                    }
                }
            }
        }
    }
}

/// The clang target triple and any extra flags the freestanding C in this tree
/// needs for `arch`. Every C source here is compiled for the kernel rather than
/// for the machine doing the build, so the triple follows the cargo target.
/// `None` where there is no kernel C for that architecture.
///
/// The two x86_64 flags have no counterpart elsewhere. Kernel code cannot use the
/// red zone below the stack pointer, because an interrupt frame would land in it,
/// and the kernel code model places the image in the top 2GB. aarch64 addresses
/// the whole space uniformly and needs neither.
fn c_target(arch: &str) -> Option<(&'static str, &'static [&'static str])> {
    match arch {
        "x86_64" => Some(("x86_64-unknown-none-elf", &["-mno-red-zone", "-mcmodel=kernel"][..])),
        "aarch64" => Some(("aarch64-unknown-none-elf", &[][..])),
        "riscv64" => Some(("riscv64-unknown-none-elf", &[][..])),
        _ => None,
    }
}

/// The user target whose capsule binaries this kernel embeds.
///
/// The build system passes `NONOS_USER_TARGET` so the capsules the kernel bakes
/// in are built for the same architecture it is. Defaults to the x86_64 user
/// target, which is what a plain `cargo build` with no make wrapper expects.
fn user_target() -> String {
    println!("cargo:rerun-if-env-changed=NONOS_USER_TARGET");
    let target = env::var("NONOS_USER_TARGET").unwrap_or_else(|_| "x86_64-nonos-user".to_string());
    // The embed sites are `include_bytes!`, which takes a literal, so the path
    // has to be assembled at compile time. Re-exporting the value as a rustc env
    // lets them reach it through `env!` inside a `concat!`.
    println!("cargo:rustc-env=NONOS_USER_TARGET={target}");
    target
}
