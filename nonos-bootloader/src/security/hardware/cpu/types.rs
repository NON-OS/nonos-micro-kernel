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

#[derive(Debug, Clone, Copy, Default)]
pub struct CpuSecurityFeatures {
    pub smep: bool,
    pub smap: bool,
    pub nx_bit: bool,
    pub aes_ni: bool,
    pub rdrand: bool,
    pub rdseed: bool,
    pub sha_ext: bool,
    pub tpm_support: bool,
    pub umip: bool,
    pub ibrs: bool,
    pub stibp: bool,
}

impl CpuSecurityFeatures {
    pub fn has_hardware_rng(&self) -> bool {
        self.rdrand || self.rdseed
    }
    pub fn has_exploit_mitigations(&self) -> bool {
        self.smep && self.smap && self.nx_bit
    }
    pub fn has_spectre_mitigations(&self) -> bool {
        self.ibrs && self.stibp
    }
}
