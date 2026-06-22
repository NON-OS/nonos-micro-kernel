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

pub fn capsule_ctx(elf: &[u8], caps: u64, epoch: u64) -> [u8; 48] {
    let mut out = [0u8; 48];
    out[..32].copy_from_slice(blake3::hash(elf).as_bytes());
    out[32..40].copy_from_slice(&caps.to_be_bytes());
    out[40..48].copy_from_slice(&epoch.to_be_bytes());
    out
}
