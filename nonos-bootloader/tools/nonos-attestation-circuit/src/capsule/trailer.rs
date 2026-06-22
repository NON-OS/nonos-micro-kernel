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

pub fn capsule_trailer(proof: &[u8]) -> Result<Vec<u8>, String> {
    if proof.get(128).copied() != Some(8) {
        return Err("capsule proof depth must be 8".into());
    }
    let mut out = Vec::with_capacity(8 + proof.len());
    out.extend_from_slice(b"NZKCAPS2");
    out.extend_from_slice(proof);
    Ok(out)
}
