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

use std::io::Write;
use std::path::Path;

use base64::Engine;

use nonos_attestation_circuit::nox::{check_receipt, hex32};

use super::args::Args;
use super::spool::spool_paths;
use super::submission::Submission;

pub fn handle(body: &[u8], args: &Args) -> Result<String, String> {
    let sub: Submission =
        serde_json::from_slice(body).map_err(|e| format!("bad submission json: {e}"))?;
    let artifact = if sub.artifact_b64.is_empty() {
        Vec::new()
    } else {
        base64::engine::general_purpose::STANDARD
            .decode(sub.artifact_b64.as_bytes())
            .map_err(|e| format!("bad artifact base64: {e}"))?
    };
    let (receipt_path, artifact_path) = spool_paths(Path::new(&args.spool), &sub.receipt)?;
    let tmp = tempfile(&artifact)?;
    let artifact_arg = if artifact.is_empty() { None } else { Some(tmp.as_path()) };
    let verifier = check_receipt(
        &sub.receipt,
        Path::new(&args.verifying_key),
        Path::new(&args.transcript),
        artifact_arg,
    )?;
    if let Some(parent) = receipt_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_vec_pretty(&sub.receipt).map_err(|e| e.to_string())?;
    std::fs::write(&receipt_path, pretty).map_err(|e| e.to_string())?;
    if !artifact.is_empty() {
        std::fs::write(&artifact_path, &artifact).map_err(|e| e.to_string())?;
    }
    let _ = std::fs::remove_file(&tmp);
    Ok(format!("{{\"accepted\":true,\"verifier_hash\":\"{}\"}}", hex32(&verifier)))
}

fn tempfile(artifact: &[u8]) -> Result<std::path::PathBuf, String> {
    let mut path = std::env::temp_dir();
    let mut tag = [0u8; 8];
    getrandom_fill(&mut tag)?;
    path.push(format!("nox-artifact-{}", hex::encode(tag)));
    let mut f = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    f.write_all(artifact).map_err(|e| e.to_string())?;
    Ok(path)
}

fn getrandom_fill(buf: &mut [u8]) -> Result<(), String> {
    use rand::RngCore;
    rand::thread_rng().fill_bytes(buf);
    Ok(())
}
