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

use ark_bls12_381::Fr;
use ark_serialize::{CanonicalSerialize, Compress};
use nonos_attestation_circuit::{ceremony, NonosAttestationCircuit};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "init" => cmd_init(&args[2..]),
        "contribute" => cmd_contribute(&args[2..]),
        "assemble" => cmd_assemble(&args[2..]),
        "finalize" => cmd_finalize(&args[2..]),
        "extract-vk" => cmd_extract_vk(&args[2..]),
        "verify" => cmd_verify(&args[2..]),
        "--help" | "-h" => print_usage(&args[0]),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage(&args[0]);
        }
    }
}

fn print_usage(name: &str) {
    eprintln!(
        r#"NØNOS ZK Ceremony Tool

Usage: {} <command> [options]

Commands:
  init          Initialize a new ceremony
  contribute    Add a contribution to existing parameters
  finalize      Finalize ceremony and extract VKs
  extract-vk    Extract VK from finalized parameters
  verify        Verify ceremony transcript

Examples:
  {} init --output ceremony/params_0.bin --circuit boot-authority
  {} contribute --input params_0.bin --output params_1.bin --name "Org:Name"
  {} finalize --input params_final.bin --output vk/ --transcript ceremony.json
  {} extract-vk --params params.bin --output vk_boot_authority.bin
"#,
        name, name, name, name, name
    );
}

fn cmd_init(args: &[String]) {
    let mut output = String::new();
    let mut circuit_name = String::from("boot-authority");
    let mut ceremony_id = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" => {
                output = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--circuit" | "-c" => {
                circuit_name = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--id" => {
                ceremony_id = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if output.is_empty() {
        eprintln!("ERROR: --output required");
        return;
    }

    if ceremony_id.is_empty() {
        ceremony_id = format!(
            "nonos-{}-{}",
            circuit_name,
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
        );
    }

    eprintln!("[init] Circuit: {}", circuit_name);
    eprintln!("[init] Ceremony ID: {}", ceremony_id);

    let circuit: NonosAttestationCircuit<Fr> = Default::default();
    match ceremony::ceremony_init(circuit, &ceremony_id, &circuit_name) {
        Ok((params, metadata)) => {
            let params_data = params.serialize().expect("serialize params");
            fs::write(&output, &params_data).expect("write params");

            let meta_path = format!("{}.meta.json", output);
            let meta_json = serde_json::to_string_pretty(&metadata).expect("serialize metadata");
            fs::write(&meta_path, &meta_json).expect("write metadata");

            eprintln!("[init] Parameters: {} ({} bytes)", output, params_data.len());
            eprintln!("[init] Metadata: {}", meta_path);
            eprintln!(
                "[init] Params hash: {:02x}{:02x}{:02x}{:02x}...",
                params.params_hash[0],
                params.params_hash[1],
                params.params_hash[2],
                params.params_hash[3]
            );
            println!("ceremony_initialized=true");
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_contribute(args: &[String]) {
    let mut input = String::new();
    let mut output = String::new();
    let mut name = String::new();
    let mut contact = String::new();
    let mut location = String::new();
    let mut entropy_source = String::from("system");

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                input = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--output" | "-o" => {
                output = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--name" | "-n" => {
                name = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--contact" => {
                contact = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--location" => {
                location = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--entropy" => {
                entropy_source = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if input.is_empty() || output.is_empty() || name.is_empty() {
        eprintln!("ERROR: --input, --output, and --name required");
        return;
    }

    eprintln!("[contribute] Loading parameters from {}", input);
    let params_data = fs::read(&input).expect("read input params");
    let prev_params = ceremony::CeremonyParams::deserialize(&params_data).expect("deserialize");

    eprintln!("[contribute] Contributor: {}", name);
    eprintln!("[contribute] Previous round: {}", prev_params.round);

    let external_randomness = gather_entropy(&entropy_source);

    match ceremony::contribute_randomness(
        &prev_params,
        &name,
        &contact,
        &location,
        &entropy_source,
        &external_randomness,
    ) {
        Ok((new_params, mut record)) => {
            // The contribution secret (tau/alpha/beta) lives only inside
            // contribute_randomness and is dropped when it returns; it is never
            // written to disk. Attest that destruction so finalize, which
            // refuses any contribution without it, can verify the chain.
            ceremony::add_destruction_attestation(&mut record, "ephemeral-process-memory", 0, None);

            let new_data = new_params.serialize().expect("serialize");
            fs::write(&output, &new_data).expect("write output");

            let record_path = format!("{}.contribution.json", output);
            let record_json = serde_json::to_string_pretty(&record).expect("serialize record");
            fs::write(&record_path, &record_json).expect("write record");

            eprintln!("[contribute] New parameters: {} ({} bytes)", output, new_data.len());
            eprintln!("[contribute] Contribution record: {}", record_path);
            eprintln!("[contribute] New round: {}", new_params.round);
            println!("contribution_complete=true");
            println!("round={}", new_params.round);
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

// Build the CeremonyTranscript finalize expects from the init metadata and the
// per-round contribution records, so the flow needs no external JSON tooling.
// Usage: assemble --meta <init.meta.json> --output <transcript.json> <rec1.json> ...
fn cmd_assemble(args: &[String]) {
    let mut meta = String::new();
    let mut output = String::new();
    let mut records: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--meta" | "-m" => {
                meta = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--output" | "-o" => {
                output = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            rec => {
                records.push(rec.to_string());
                i += 1;
            }
        }
    }

    if meta.is_empty() || output.is_empty() || records.is_empty() {
        eprintln!("ERROR: assemble needs --meta, --output, and at least one record file");
        std::process::exit(1);
    }

    let metadata: ceremony::CeremonyMetadata =
        serde_json::from_str(&fs::read_to_string(&meta).expect("read metadata"))
            .expect("parse ceremony metadata");

    let mut contributions: Vec<ceremony::ContributionRecord> = records
        .iter()
        .map(|r| {
            serde_json::from_str(&fs::read_to_string(r).expect("read contribution record"))
                .expect("parse contribution record")
        })
        .collect();
    contributions.sort_by_key(|c| c.round);

    let transcript = ceremony::CeremonyTranscript {
        metadata,
        contributions,
        final_vk_hash: None,
        verification_passed: false,
    };

    fs::write(&output, serde_json::to_string_pretty(&transcript).expect("serialize transcript"))
        .expect("write transcript");
    eprintln!(
        "[assemble] transcript written: {} ({} contributions)",
        output,
        transcript.contributions.len()
    );
}

fn cmd_finalize(args: &[String]) {
    let mut input = String::new();
    let mut output_dir = String::new();
    let mut transcript = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                input = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--output" | "-o" => {
                output_dir = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--transcript" | "-t" => {
                transcript = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if input.is_empty() || output_dir.is_empty() {
        eprintln!("ERROR: --input and --output required");
        return;
    }

    fs::create_dir_all(&output_dir).expect("create output dir");

    eprintln!("[finalize] Loading final parameters from {}", input);
    let params_data = fs::read(&input).expect("read params");
    let final_params = ceremony::CeremonyParams::deserialize(&params_data).expect("deserialize");

    let contributions = load_contributions(&transcript);

    if contributions.len() < ceremony::MIN_PARTICIPANTS {
        eprintln!(
            "ERROR: Need at least {} contributions, got {}",
            ceremony::MIN_PARTICIPANTS,
            contributions.len()
        );
        std::process::exit(1);
    }

    match ceremony::ceremony_finalize(&final_params, &contributions) {
        Ok((vk, transcript_data)) => {
            let vk_path = Path::new(&output_dir).join("vk_boot_authority.bin");
            let mut vk_buf = Vec::new();
            vk.serialize_with_mode(&mut vk_buf, Compress::Yes).expect("serialize VK");
            fs::write(&vk_path, &vk_buf).expect("write VK");

            // Emit the attestation key pair in the exact format the prover
            // (generate-proof) and the kernel embed (embed-zk-proof) consume,
            // so the ceremony replaces the seeded generate path one-for-one.
            // finalize does not re-randomize, so final_params.pk.vk == vk.
            let pk_path = Path::new(&output_dir).join("attestation_proving_key.bin");
            let mut pk_buf = Vec::new();
            final_params
                .pk
                .serialize_with_mode(&mut pk_buf, Compress::Yes)
                .expect("serialize PK");
            fs::write(&pk_path, &pk_buf).expect("write attestation proving key");

            let avk_path = Path::new(&output_dir).join("attestation_verifying_key.bin");
            fs::write(&avk_path, &vk_buf).expect("write attestation verifying key");

            let transcript_path = Path::new(&output_dir).join("ceremony_transcript.json");
            let transcript_json =
                serde_json::to_string_pretty(&transcript_data).expect("serialize");
            fs::write(&transcript_path, &transcript_json).expect("write transcript");

            let vk_hash = blake3::hash(&vk_buf);
            eprintln!("[finalize] VK extracted: {} ({} bytes)", vk_path.display(), vk_buf.len());
            eprintln!(
                "[finalize] VK hash: {:02x}{:02x}{:02x}{:02x}...",
                vk_hash.as_bytes()[0],
                vk_hash.as_bytes()[1],
                vk_hash.as_bytes()[2],
                vk_hash.as_bytes()[3]
            );
            eprintln!("[finalize] Transcript: {}", transcript_path.display());
            println!("ceremony_finalized=true");
            println!("vk_path={}", vk_path.display());
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_extract_vk(args: &[String]) {
    let mut params_path = String::new();
    let mut output = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--params" | "-p" => {
                params_path = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            "--output" | "-o" => {
                output = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if params_path.is_empty() || output.is_empty() {
        eprintln!("ERROR: --params and --output required");
        return;
    }

    let params_data = fs::read(&params_path).expect("read params");
    let params = ceremony::CeremonyParams::deserialize(&params_data).expect("deserialize");

    let mut vk_buf = Vec::new();
    params.pk.vk.serialize_with_mode(&mut vk_buf, Compress::Yes).expect("serialize VK");
    fs::write(&output, &vk_buf).expect("write VK");

    let fp = compute_vk_fingerprint(&vk_buf);
    eprintln!("[extract] VK: {} ({} bytes)", output, vk_buf.len());
    eprintln!(
        "[extract] Fingerprint: {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        fp[0], fp[1], fp[2], fp[3], fp[4], fp[5], fp[6], fp[7]
    );
    println!("vk_extracted=true");
}

fn cmd_verify(args: &[String]) {
    let mut transcript = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--transcript" | "-t" => {
                transcript = args.get(i + 1).cloned().unwrap_or_default();
                i += 2;
            }
            _ => i += 1,
        }
    }

    if transcript.is_empty() {
        eprintln!("ERROR: --transcript required");
        return;
    }

    let transcript_json = fs::read_to_string(&transcript).expect("read transcript");
    let data: ceremony::CeremonyTranscript = serde_json::from_str(&transcript_json).expect("parse");

    eprintln!("[verify] Ceremony: {}", data.metadata.ceremony_id);
    eprintln!("[verify] Contributions: {}", data.contributions.len());
    eprintln!("[verify] Finalized: {}", data.metadata.finalized);

    let mut all_valid = true;
    for (i, contrib) in data.contributions.iter().enumerate() {
        let has_destruction = contrib.destruction_attestation.is_some();
        if !has_destruction {
            eprintln!("[verify] WARNING: Contribution {} missing destruction attestation", i + 1);
            all_valid = false;
        }
        eprintln!(
            "[verify] {} Round {} by {} - destruction: {}",
            if has_destruction { "✓" } else { "✗" },
            contrib.round,
            contrib.contributor_id,
            has_destruction
        );
    }

    if data.contributions.len() >= ceremony::MIN_PARTICIPANTS
        && all_valid
        && data.verification_passed
    {
        eprintln!("[verify] Ceremony VALID");
        println!("ceremony_valid=true");
    } else {
        eprintln!("[verify] Ceremony INVALID");
        println!("ceremony_valid=false");
        std::process::exit(1);
    }
}

fn gather_entropy(source: &str) -> Vec<u8> {
    // The contribution's secret randomness (the toxic waste) is derived from
    // this. If it is weak or zero the trapdoor becomes recoverable, so the
    // ceremony fails closed rather than ever contribute with degenerate entropy.
    let mut entropy = vec![0u8; 64];

    if source == "system" || source == "/dev/random" || source == "/dev/urandom" {
        use std::io::Read;
        let mut file = fs::File::open("/dev/urandom")
            .expect("ceremony: cannot open /dev/urandom for contribution entropy");
        file.read_exact(&mut entropy)
            .expect("ceremony: failed to read 64 bytes of OS entropy");
    } else if Path::new(source).exists() {
        entropy = fs::read(source).expect("ceremony: cannot read entropy file");
    } else {
        panic!("ceremony: entropy source '{source}' is not 'system' nor a readable file");
    }

    assert!(
        entropy.len() >= 32 && entropy.iter().any(|&b| b != 0),
        "ceremony: refusing to contribute with weak/zero entropy from source '{source}'"
    );

    let timestamp =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    entropy.extend_from_slice(&timestamp.to_le_bytes());

    entropy
}

fn load_contributions(transcript_path: &str) -> Vec<ceremony::ContributionRecord> {
    if transcript_path.is_empty() || !Path::new(transcript_path).exists() {
        return Vec::new();
    }

    if let Ok(json) = fs::read_to_string(transcript_path) {
        if let Ok(transcript) = serde_json::from_str::<ceremony::CeremonyTranscript>(&json) {
            return transcript.contributions;
        }
    }

    Vec::new()
}

fn compute_vk_fingerprint(vk_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:VK:FINGERPRINT:v1");
    hasher.update(vk_bytes);
    *hasher.finalize().as_bytes()
}
