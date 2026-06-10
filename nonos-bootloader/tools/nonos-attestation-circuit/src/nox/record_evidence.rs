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

pub fn record_evidence_hash<T: Serialize>(record: &T) -> Result<[u8; 32], serde_json::Error> {
    let bytes = serde_json::to_vec(record)?;
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:NOX:ZK:RECORD:v1");
    hasher.update(&bytes);
    Ok(*hasher.finalize().as_bytes())
}
