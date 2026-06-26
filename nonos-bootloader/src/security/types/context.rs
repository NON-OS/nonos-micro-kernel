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

#[derive(Debug, Default, Clone)]
pub struct SecurityContext {
    pub secure_boot_enabled: bool,
    pub platform_key_verified: bool,
    pub signature_database_valid: bool,
    pub hardware_rng_available: bool,
    pub ed25519_health_ok: bool,
    pub blake3_health_ok: bool,
    pub production_keys_loaded: bool,
    pub key_count: usize,
    pub measured_boot_active: bool,
    pub tpm_counter_ok: bool,
}

impl SecurityContext {
    pub const fn new() -> Self {
        Self {
            secure_boot_enabled: false,
            platform_key_verified: false,
            signature_database_valid: false,
            hardware_rng_available: false,
            ed25519_health_ok: false,
            blake3_health_ok: false,
            production_keys_loaded: false,
            key_count: 0,
            measured_boot_active: false,
            tpm_counter_ok: false,
        }
    }
}
