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

use super::record::{ContributionRecord, DestructionAttestation};

pub fn add_destruction_attestation(
    record: &mut ContributionRecord,
    method: &str,
    witness_count: usize,
    video_hash: Option<[u8; 32]>,
) {
    let attestation_data =
        format!("{}:{}:{}:{}", record.contributor_id, record.round, method, witness_count);
    let attestation_hash = *blake3::hash(attestation_data.as_bytes()).as_bytes();
    record.destruction_attestation = Some(DestructionAttestation {
        method: method.to_string(),
        witness_count,
        attestation_hash,
        video_hash,
    });
}
