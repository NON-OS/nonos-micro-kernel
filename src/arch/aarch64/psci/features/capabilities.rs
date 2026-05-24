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

use super::query;
use super::version::{psci_version, PsciVersion};

#[derive(Debug, Clone)]
pub struct PsciCapabilities {
    pub version: PsciVersion,
    pub cpu_suspend: bool,
    pub cpu_off: bool,
    pub cpu_on: bool,
    pub affinity_info: bool,
    pub system_off: bool,
    pub system_reset: bool,
    pub system_reset2: bool,
    pub system_suspend: bool,
    pub mem_protect: bool,
}

impl PsciCapabilities {
    pub fn discover() -> Self {
        Self {
            version: psci_version(),
            cpu_suspend: query::has_cpu_suspend(),
            cpu_off: query::has_cpu_off(),
            cpu_on: query::has_cpu_on(),
            affinity_info: query::has_affinity_info(),
            system_off: query::has_system_off(),
            system_reset: query::has_system_reset(),
            system_reset2: query::has_system_reset2(),
            system_suspend: query::has_system_suspend(),
            mem_protect: query::has_mem_protect(),
        }
    }
}
