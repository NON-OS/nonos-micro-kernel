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
use nonos_attestation_circuit::policy_tree::{field_bytes, split_hash, witness, PolicyWitness};

use super::args::Args;
use super::hash_file::hash_file;
use super::policy_entry::PolicyEntry;
use super::policy_name::policy_name;
use super::read_policy_file::read_policy_file;

pub fn policy_witness(
    args: &Args,
    capsule_hash: [u8; 32],
    caps: u64,
) -> Result<(PolicyWitness<Fr>, [u8; 32]), String> {
    let name = policy_name(args)?;
    let entries = match &args.policy_file {
        Some(path) => read_policy_file(path)?,
        None => vec![PolicyEntry { name: name.clone(), capsule: args.capsule.clone(), caps }],
    };
    let mut fields = Vec::with_capacity(entries.len());
    let mut index = None;
    for (i, entry) in entries.iter().enumerate() {
        let (hash, _) = hash_file(&entry.capsule)?;
        if entry.name == name {
            index = Some(i);
            if hash != capsule_hash || entry.caps != caps {
                return Err("policy entry does not match capsule".into());
            }
        }
        let (hi, lo) = split_hash::<Fr>(&hash);
        fields.push((hi, lo, entry.caps));
    }
    let proof = witness(&fields, index.ok_or("policy entry missing")?)?;
    let root = field_bytes(&proof.root);
    Ok((proof, root))
}
