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

use serde::Serialize;

#[derive(Serialize)]
pub struct Receipt {
    pub schema_version: u32,
    pub kind: &'static str,
    pub chain_id: u64,
    pub circuit_name: String,
    pub circuit_id: String,
    pub contributor: String,
    pub contributor_address: String,
    pub round: u32,
    pub policy_epoch: u64,
    pub vk_sha256: String,
    pub transcript_sha256: String,
    pub transcript_final_vk_hash: String,
    pub previous_params_hash: String,
    pub new_params_hash: String,
    pub randomness_commitment: String,
    pub destruction_attestation_hash: Option<String>,
    pub contribution_timestamp: u64,
    pub evidence_hash: String,
    pub receipt_id: String,
    pub uri: String,
}
