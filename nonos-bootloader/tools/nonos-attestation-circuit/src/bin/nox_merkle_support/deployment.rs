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

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Deployment {
    pub chain_id: u64,
    pub contracts: DeployedContracts,
}

#[derive(Deserialize, Clone)]
pub struct DeployedContracts {
    pub reward_pool: String,
    pub reward_root_manager: String,
}

pub fn load_deployment(path: &Path) -> Result<Deployment, Box<dyn std::error::Error>> {
    let d: Deployment = serde_json::from_slice(&std::fs::read(path)?)?;
    Ok(d)
}
