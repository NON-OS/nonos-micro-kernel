// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use std::{fs, fs::File, io::Write, path::PathBuf, process::Command};

use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress};
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};
use blake3::Hasher as Blake3;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use nonos_attestation_circuit::{expected_program_hash_bytes, NonosAttestationCircuit};

#[derive(Parser, Debug)]
#[command(name = "generate-keys", about = "NONOS attestation keys (Groth16, BLS12-381)")]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    Generate {
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        output: String,

        #[arg(short = 's', long = "seed", value_name = "SEED")]
        seed: Option<String>,

        #[arg(long = "print-program-hash")]
        print_program_hash: bool,

        #[arg(long = "sign-key", value_name = "PATH")]
        sign_key: Option<PathBuf>,

        #[arg(long = "bundle-out", value_name = "PATH")]
        bundle_out: Option<PathBuf>,

        #[arg(long = "allow-unsigned", action = clap::ArgAction::SetTrue)]
        allow_unsigned: bool,

        #[arg(long = "ceremony-dir", value_name = "DIR")]
        ceremony_dir: Option<PathBuf>,
    },

    ExtractVk {
        #[arg(long, value_name = "PATH")]
        pk: PathBuf,

        #[arg(long, value_name = "PATH", default_value = "attestation_verifying_key.bin")]
        out: PathBuf,
    },

    InspectVk {
        #[arg(long, value_name = "PATH")]
        vk: PathBuf,
    },
}

#[derive(Serialize, Deserialize)]
struct Metadata {
    tool: String,
    tool_version: String,
    rustc_version: String,
    cargo_version: String,
    commit: Option<String>,
    seed: String,
    vk_blake3: String,
    public_inputs_expected: usize,
    canonical_vk_len: usize,
    generated_at_utc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ceremony: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contributors: Option<Vec<JsonValue>>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    match args.cmd {
        Cmd::Generate {
            output,
            seed,
            print_program_hash,
            sign_key,
            bundle_out,
            allow_unsigned,
            ceremony_dir,
        } => {
            let out_dir = PathBuf::from(output);
            if !out_dir.exists() {
                fs::create_dir_all(&out_dir)
                    .map_err(|e| format!("mkdir {}: {e}", out_dir.display()))?;
            }

            let (seed_u64, seed_str) = match seed {
                Some(ref s) => (parse_seed(s), s.clone()),
                None => {
                    use std::time::{SystemTime, UNIX_EPOCH};
                    let now =
                        SystemTime::now().duration_since(UNIX_EPOCH).expect("time").as_nanos();
                    let entropy = format!("{:x}{:x}", now, std::process::id());
                    let hash = blake3::hash(entropy.as_bytes());
                    let val = u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap());
                    (val, format!("auto:{:016x}", val))
                }
            };
            let mut rng = StdRng::seed_from_u64(seed_u64);

            let circuit: NonosAttestationCircuit<Fr> = Default::default();
            let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng)
                .map_err(|e| format!("setup: {e}"))?;

            let mut pk_bytes = Vec::new();
            pk.serialize_with_mode(&mut pk_bytes, Compress::Yes)
                .map_err(|e| format!("pk serialize: {e}"))?;

            let mut vk_bytes = Vec::new();
            vk.serialize_with_mode(&mut vk_bytes, Compress::Yes)
                .map_err(|e| format!("vk serialize: {e}"))?;

            let pk_path = out_dir.join("attestation_proving_key.bin");
            let vk_path = out_dir.join("attestation_verifying_key.bin");

            write_bin(&pk_path, &pk_bytes)?;
            write_bin(&vk_path, &vk_bytes)?;

            let fp = blake3_hex(&vk_bytes);
            let inputs = vk.gamma_abc_g1.len().saturating_sub(1);

            let tool = "nonos-attestation-circuit".to_string();
            let tool_version = env!("CARGO_PKG_VERSION").to_string();
            let rustc_version = get_rustc_version();
            let cargo_version = get_cargo_version();
            let commit = std::env::var("GIT_COMMIT").ok().or_else(git_commit_hash);
            let ts = chrono::Utc::now().to_rfc3339();

            let (ceremony_json, contributors) =
                if let Some(dir) = ceremony_dir { read_ceremony_dir(&dir)? } else { (None, None) };

            let metadata = Metadata {
                tool,
                tool_version,
                rustc_version,
                cargo_version,
                commit,
                seed: seed_str.clone(),
                vk_blake3: fp.clone(),
                public_inputs_expected: inputs,
                canonical_vk_len: vk_bytes.len(),
                generated_at_utc: ts,
                ceremony: ceremony_json,
                contributors,
            };

            let metadata_json =
                serde_json::to_vec_pretty(&metadata).map_err(|e| format!("metadata json: {e}"))?;
            write_bin(&out_dir.join("metadata.json"), &metadata_json)?;

            println!("NONOS attestation keys generated");
            println!("proving_key:   {} ({} bytes)", pk_path.display(), pk_bytes.len());
            println!("verifying_key: {} ({} bytes)", vk_path.display(), vk_bytes.len());
            println!("vk_blake3:     {}", fp);
            println!("public_inputs_expected: {}", inputs);

            if print_program_hash {
                let ph = expected_program_hash_bytes();
                println!("program_hash_hex: {}", hex::encode(ph));
            }

            let bundle_path =
                bundle_out.unwrap_or_else(|| out_dir.join("attestation_bundle.tar.gz"));
            if sign_key.is_some() {
                let sig = sign_bundle(sign_key.as_ref().unwrap(), &vk_bytes, &metadata_json)?;
                write_bin(&out_dir.join("signature.sig"), &sig)?;
                create_bundle(
                    &bundle_path,
                    &vk_path,
                    &out_dir.join("metadata.json"),
                    &out_dir.join("signature.sig"),
                )?;
                println!("signed bundle written: {}", bundle_path.display());
            } else if allow_unsigned {
                create_bundle(
                    &bundle_path,
                    &vk_path,
                    &out_dir.join("metadata.json"),
                    &PathBuf::from(""),
                )?;
                println!("unsigned bundle written: {}", bundle_path.display());
            } else {
                println!("No signing key provided; bundle not signed.");
            }

            Ok(())
        }

        Cmd::ExtractVk { pk, out } => {
            let pk = read_pk_any(&pk)?;
            let vk = pk.vk;

            let mut vk_bytes = Vec::new();
            vk.serialize_with_mode(&mut vk_bytes, Compress::Yes)
                .map_err(|e| format!("vk serialize: {e}"))?;
            write_bin(&out, &vk_bytes)?;

            let fp = blake3_hex(&vk_bytes);
            let inputs = vk.gamma_abc_g1.len().saturating_sub(1);

            println!("verifying_key_written: {} ({} bytes)", out.display(), vk_bytes.len());
            println!("vk_blake3: {}", fp);
            println!("public_inputs_expected: {}", inputs);
            Ok(())
        }

        Cmd::InspectVk { vk } => {
            let vk = read_vk_any(&vk)?;
            let mut comp = Vec::new();
            vk.serialize_with_mode(&mut comp, Compress::Yes)
                .map_err(|e| format!("vk serialize: {e}"))?;

            let fp = blake3_hex(&comp);
            let inputs = vk.gamma_abc_g1.len().saturating_sub(1);

            println!("vk_ok");
            println!("canonical_compressed_len: {}", comp.len());
            println!("vk_blake3: {}", fp);
            println!("public_inputs_expected: {}", inputs);
            Ok(())
        }
    }
}

fn read_ceremony_dir(dir: &PathBuf) -> Result<(Option<JsonValue>, Option<Vec<JsonValue>>), String> {
    if !dir.exists() {
        return Ok((None, None));
    }
    let mut contributors = Vec::new();
    let mut ceremony_meta: Option<JsonValue> = None;
    for entry in fs::read_dir(dir).map_err(|e| format!("read ceremony dir: {e}"))? {
        let p = entry.map_err(|e| format!("read dir entry: {e}"))?.path();
        if p.is_file() {
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name.ends_with(".json") {
                    let b = fs::read(&p).map_err(|e| format!("read ceremony file: {e}"))?;
                    let v: JsonValue =
                        serde_json::from_slice(&b).map_err(|e| format!("parse json: {e}"))?;
                    if name == "ceremony.json" {
                        ceremony_meta = Some(v);
                    } else {
                        contributors.push(v);
                    }
                }
            }
        }
    }
    Ok((ceremony_meta, if contributors.is_empty() { None } else { Some(contributors) }))
}

fn write_bin(path: &PathBuf, bytes: &[u8]) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return Err("empty path".into());
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("mkdir {}: {e}", parent.display()))?;
        }
    }
    let mut f = File::create(path).map_err(|e| format!("create {}: {e}", path.display()))?;
    f.write_all(bytes).map_err(|e| format!("write {}: {e}", path.display()))
}

fn read_vk_any(path: &PathBuf) -> Result<VerifyingKey<Bls12_381>, String> {
    use ark_serialize::Validate;
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if bytes.is_empty() {
        return Err("vk file is empty".into());
    }
    VerifyingKey::<Bls12_381>::deserialize_with_mode(
        &mut ark_std::io::Cursor::new(&bytes),
        Compress::Yes,
        Validate::Yes,
    )
    .or_else(|_| {
        VerifyingKey::<Bls12_381>::deserialize_with_mode(
            &mut ark_std::io::Cursor::new(&bytes),
            Compress::No,
            Validate::Yes,
        )
    })
    .map_err(|_| "not a valid arkworks Groth16 VK (BLS12-381)".to_string())
}

fn read_pk_any(path: &PathBuf) -> Result<ProvingKey<Bls12_381>, String> {
    use ark_serialize::Validate;
    let bytes = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    if bytes.is_empty() {
        return Err("pk file is empty".into());
    }
    ProvingKey::<Bls12_381>::deserialize_with_mode(
        &mut ark_std::io::Cursor::new(&bytes),
        Compress::Yes,
        Validate::Yes,
    )
    .or_else(|_| {
        ProvingKey::<Bls12_381>::deserialize_with_mode(
            &mut ark_std::io::Cursor::new(&bytes),
            Compress::No,
            Validate::Yes,
        )
    })
    .map_err(|_| "not a valid arkworks Groth16 ProvingKey (BLS12-381)".to_string())
}

fn parse_seed(s: &str) -> u64 {
    if let Ok(v) = s.parse::<u64>() {
        return v;
    }
    let s = s.trim_start_matches("0x");
    let mut acc = 0u64;
    for &b in s.as_bytes() {
        let v = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => 10 + (b - b'a'),
            b'A'..=b'F' => 10 + (b - b'A'),
            _ => 0,
        } as u64;
        acc = acc.wrapping_mul(257).wrapping_add(v);
    }
    if acc == 0 {
        42
    } else {
        acc
    }
}

fn blake3_hex(bytes: &[u8]) -> String {
    let mut h = Blake3::new();
    h.update(bytes);
    h.finalize().to_hex().to_string()
}

fn git_commit_hash() -> Option<String> {
    if let Ok(out) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if out.status.success() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return Some(s.trim().to_string());
            }
        }
    }
    None
}

fn get_rustc_version() -> String {
    std::env::var("RUSTC_VERSION").unwrap_or_else(|_| {
        std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".into())
    })
}

fn get_cargo_version() -> String {
    std::env::var("CARGO_VERSION").unwrap_or_else(|_| {
        std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".into())
    })
}

fn create_bundle(
    bundle_out: &PathBuf,
    vk_path: &PathBuf,
    metadata_path: &PathBuf,
    signature_path: &PathBuf,
) -> Result<(), String> {
    use flate2::{write::GzEncoder, Compression};
    use tar::Builder;

    let f = File::create(bundle_out).map_err(|e| format!("create bundle: {e}"))?;
    let enc = GzEncoder::new(f, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_path_with_name(vk_path, "attestation_verifying_key.bin")
        .map_err(|e| format!("tar vk: {e}"))?;
    tar.append_path_with_name(metadata_path, "metadata.json")
        .map_err(|e| format!("tar metadata: {e}"))?;
    if signature_path.exists() && signature_path.metadata().map_err(|e| e.to_string())?.len() > 0 {
        tar.append_path_with_name(signature_path, "signature.sig")
            .map_err(|e| format!("tar sig: {e}"))?;
    }

    tar.finish().map_err(|e| format!("finish tar: {e}"))?;
    Ok(())
}

fn sign_bundle(
    sign_key_path: &PathBuf,
    vk_bytes: &[u8],
    metadata_json: &[u8],
) -> Result<Vec<u8>, String> {
    use ed25519_dalek::{Signer, SigningKey};

    let key_bytes = fs::read(sign_key_path).map_err(|e| format!("read sign key: {e}"))?;

    let signing_key = if key_bytes.len() == 64 {
        let seed: [u8; 32] = key_bytes[..32].try_into().unwrap();
        SigningKey::from_bytes(&seed)
    } else if key_bytes.len() == 32 {
        let seed: [u8; 32] = key_bytes.try_into().unwrap();
        SigningKey::from_bytes(&seed)
    } else if let Ok(s) = std::str::from_utf8(&key_bytes) {
        let s = s.trim();
        let raw = hex::decode(s).map_err(|e| format!("hex decode sign key: {e}"))?;
        if raw.len() == 32 {
            let seed: [u8; 32] = raw.try_into().unwrap();
            SigningKey::from_bytes(&seed)
        } else if raw.len() == 64 {
            let seed: [u8; 32] = raw[..32].try_into().unwrap();
            SigningKey::from_bytes(&seed)
        } else {
            return Err("sign key must be 32-byte secret or 64-byte keypair (raw or hex)".into());
        }
    } else {
        return Err("sign key must be 32-byte secret or 64-byte keypair (raw or hex)".into());
    };

    let mut signed_input = Vec::with_capacity(vk_bytes.len() + metadata_json.len());
    signed_input.extend_from_slice(vk_bytes);
    signed_input.extend_from_slice(metadata_json);

    let signature = signing_key.sign(&signed_input);
    Ok(signature.to_bytes().to_vec())
}
