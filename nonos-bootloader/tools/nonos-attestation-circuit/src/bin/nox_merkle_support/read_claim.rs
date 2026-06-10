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

use std::path::Path;

use nonos_attestation_circuit::nox::address_bytes;

pub struct ClaimFacts {
    pub contributor: String,
    pub contributor_bytes: [u8; 20],
    pub receipt_id: String,
    pub receipt_id_bytes: [u8; 32],
    pub circuit_id: String,
    pub circuit_id_bytes: [u8; 32],
}

pub fn read_claim(path: &Path) -> Result<ClaimFacts, Box<dyn std::error::Error>> {
    let receipt: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let contributor = field(&receipt, "contributor_address")?;
    let receipt_id = field(&receipt, "receipt_id")?;
    let circuit_id = field(&receipt, "circuit_id")?;
    Ok(ClaimFacts {
        contributor_bytes: address_bytes(&contributor)?,
        receipt_id_bytes: hex32_bytes(&receipt_id)?,
        circuit_id_bytes: hex32_bytes(&circuit_id)?,
        contributor,
        receipt_id,
        circuit_id,
    })
}

fn field(receipt: &serde_json::Value, name: &str) -> Result<String, String> {
    receipt
        .get(name)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("receipt is missing field {name}"))
}

pub fn hex32_bytes(text: &str) -> Result<[u8; 32], String> {
    if text.len() != 66 || !text.starts_with("0x") {
        return Err(format!("{text} is not a 32-byte 0x hex value"));
    }
    let mut out = [0u8; 32];
    hex::decode_to_slice(&text[2..], &mut out).map_err(|e| e.to_string())?;
    Ok(out)
}
