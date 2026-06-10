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
pub struct RootOutput {
    pub epoch: u64,
    pub pool_id: String,
    pub root: String,
    pub leaf_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_root_manager: Option<String>,
}

#[derive(Serialize)]
pub struct ClaimOutput {
    pub contributor: String,
    pub receipt_id: String,
    pub circuit_id: String,
    pub amount: String,
    pub epoch: u64,
    pub pool_id: String,
    pub leaf: String,
    pub proof: Vec<String>,
}
