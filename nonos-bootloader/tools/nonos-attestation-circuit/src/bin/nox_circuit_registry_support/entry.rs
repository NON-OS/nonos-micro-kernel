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
pub struct RegistryEntry {
    pub schema_version: u32,
    pub circuit_name: String,
    pub circuit_id: String,
    pub source_tree_hash: String,
    pub cargo_lock_hash: String,
    pub public_input_layout_hash: String,
    pub vk_sha256: String,
    pub transcript_sha256: String,
    pub policy_epoch: u64,
    pub status: &'static str,
    pub uri: String,
}
