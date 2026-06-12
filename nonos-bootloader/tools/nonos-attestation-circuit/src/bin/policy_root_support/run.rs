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
use clap::Parser;
use nonos_attestation_circuit::policy_tree::{field_bytes, split_hash, witness};

use super::args::Args;
use super::hash_file::hash_file;
use super::read_policy::read_policy;

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let entries = read_policy(&args.policy_file)?;
    let mut fields = Vec::with_capacity(entries.len());
    for entry in entries {
        let hash = hash_file(&entry.capsule)?;
        let (hi, lo) = split_hash::<Fr>(&hash);
        fields.push((hi, lo, entry.caps));
    }
    let proof = witness(&fields, 0)?;
    let root = field_bytes(&proof.root);
    std::fs::write(&args.output, root).map_err(|e| format!("write root: {e}"))?;
    println!("policy_root: {}", hex::encode(root));
    Ok(())
}
