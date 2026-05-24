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

use super::address::napot_addr;
use super::config::PmpConfig;
use super::error::PmpResult;

#[derive(Debug, Clone, Copy)]
pub struct PmpEntry {
    pub addr: u64,
    pub config: PmpConfig,
}

impl PmpEntry {
    pub const fn new(addr: u64, config: PmpConfig) -> Self {
        Self { addr, config }
    }

    pub fn napot(base: u64, size: u64, config: PmpConfig) -> PmpResult<Self> {
        Ok(Self { addr: napot_addr(base, size)?, config })
    }
}
