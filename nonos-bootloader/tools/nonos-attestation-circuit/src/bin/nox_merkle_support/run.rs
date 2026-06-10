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

use std::collections::BTreeSet;
use std::path::Path;

use nonos_attestation_circuit::nox::{hex32, write_json};

use super::amount::parse_amount;
use super::args::parse;
use super::deployment::load_deployment;
use super::input::ClaimsInput;
use super::leaf::claim_leaf;
use super::output::{ClaimOutput, RootOutput};
use super::read_claim::{hex32_bytes, read_claim};
use super::tree::{build_proof, build_root, process_proof};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    let input: ClaimsInput = serde_json::from_slice(&std::fs::read(&args.claims)?)?;
    if input.claims.is_empty() {
        return Err("claims input is empty".into());
    }
    let deployment = match &args.deployment {
        Some(path) => Some(load_deployment(Path::new(path))?),
        None => None,
    };
    let pool_id = hex32_bytes(&input.pool_id)?;
    let mut seen = BTreeSet::new();
    let mut rows = Vec::with_capacity(input.claims.len());
    for claim in &input.claims {
        let facts = read_claim(Path::new(&claim.receipt))?;
        if !seen.insert(facts.receipt_id_bytes) {
            return Err(format!("duplicate receipt {}", facts.receipt_id).into());
        }
        let amount = parse_amount(&claim.amount)?;
        let leaf = claim_leaf(
            &facts.contributor_bytes,
            &facts.receipt_id_bytes,
            &facts.circuit_id_bytes,
            &amount,
            input.epoch,
            &pool_id,
        );
        rows.push((facts, claim.amount.clone(), leaf));
    }
    rows.sort_by(|a, b| a.2.cmp(&b.2));
    let leaves: Vec<[u8; 32]> = rows.iter().map(|r| r.2).collect();
    let root = build_root(&leaves);
    let mut claims_out = Vec::with_capacity(rows.len());
    for (index, (facts, amount, leaf)) in rows.iter().enumerate() {
        let proof = build_proof(&leaves, index);
        if process_proof(leaf, &proof) != root {
            return Err(format!("proof self-check failed for {}", facts.receipt_id).into());
        }
        claims_out.push(ClaimOutput {
            contributor: facts.contributor.clone(),
            receipt_id: facts.receipt_id.clone(),
            circuit_id: facts.circuit_id.clone(),
            amount: amount.clone(),
            epoch: input.epoch,
            pool_id: input.pool_id.clone(),
            leaf: hex32(leaf),
            proof: proof.iter().map(hex32).collect(),
        });
    }
    let out_dir = Path::new(&args.out_dir);
    write_json(
        &out_dir.join("root.json"),
        &RootOutput {
            epoch: input.epoch,
            pool_id: input.pool_id.clone(),
            root: hex32(&root),
            leaf_count: leaves.len(),
            chain_id: deployment.as_ref().map(|d| d.chain_id),
            reward_pool: deployment.as_ref().map(|d| d.contracts.reward_pool.clone()),
            reward_root_manager: deployment
                .as_ref()
                .map(|d| d.contracts.reward_root_manager.clone()),
        },
    )?;
    write_json(&out_dir.join("claims.json"), &claims_out)?;
    println!("{}", hex32(&root));
    Ok(())
}
