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

use crate::loader::types::memory;

#[derive(Debug, Clone, Copy)]
pub struct SecurityPolicy {
    pub enforce_wx: bool,
    pub require_signature: bool,
    pub max_kernel_size: usize,
    pub min_load_address: u64,
    pub max_load_address: u64,
    pub allow_debug_symbols: bool,
    pub require_pie: bool,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            enforce_wx: true,
            require_signature: true,
            max_kernel_size: memory::MAX_KERNEL_SIZE,
            min_load_address: memory::MIN_LOAD_ADDRESS,
            max_load_address: memory::MAX_LOAD_ADDRESS,
            allow_debug_symbols: false,
            require_pie: false,
        }
    }
}

impl SecurityPolicy {
    pub fn strict() -> Self {
        Self {
            enforce_wx: true,
            require_signature: true,
            max_kernel_size: 128 * 1024 * 1024,
            min_load_address: memory::MIN_LOAD_ADDRESS,
            max_load_address: memory::MAX_LOAD_ADDRESS,
            allow_debug_symbols: false,
            require_pie: true,
        }
    }

    pub fn development() -> Self {
        Self {
            enforce_wx: false,
            require_signature: false,
            max_kernel_size: memory::MAX_KERNEL_SIZE,
            min_load_address: memory::MIN_LOAD_ADDRESS,
            max_load_address: memory::MAX_LOAD_ADDRESS,
            allow_debug_symbols: true,
            require_pie: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct SecurityCheckResult {
    pub passed: bool,
    pub wx_violations: usize,
    pub address_violations: usize,
    pub size_violations: usize,
    pub signature_valid: bool,
    pub hash_valid: bool,
    pub warnings: usize,
}

impl SecurityCheckResult {
    pub fn pass() -> Self {
        Self { passed: true, ..Default::default() }
    }

    pub fn has_violations(&self) -> bool {
        self.wx_violations > 0 || self.address_violations > 0 || self.size_violations > 0
    }
}
