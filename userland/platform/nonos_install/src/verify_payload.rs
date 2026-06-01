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

use std::fs;
use std::path::Path;

use nonos_capsule_sign::verify::decode::decode_manifest;

pub fn verify_payload(manifest: &Path, payload: &Path) -> Result<(), String> {
    let manifest_bytes = fs::read(manifest).map_err(|e| format!("read manifest: {e}"))?;
    let decoded = decode_manifest(&manifest_bytes).map_err(|e| format!("decode manifest: {e:?}"))?;
    let payload_bytes = fs::read(payload).map_err(|e| format!("read payload: {e}"))?;
    let actual = blake3::hash(&payload_bytes);
    if actual.as_bytes() != &decoded.payload_hash {
        return Err("payload hash does not match signed manifest; package tampered".to_string());
    }
    Ok(())
}
