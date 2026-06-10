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

const DS_COMMITMENT: &str = "NONOS:CAPSULE:COMMITMENT:v1";

pub(super) fn commitment(capsule: &[u8; 32], policy: &[u8; 32], epoch: u64, caps: u64) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key(DS_COMMITMENT);
    hasher.update(capsule);
    hasher.update(policy);
    hasher.update(&epoch.to_be_bytes());
    hasher.update(&caps.to_be_bytes());
    *hasher.finalize().as_bytes()
}
