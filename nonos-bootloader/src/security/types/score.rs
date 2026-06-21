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

use super::context::SecurityContext;

impl SecurityContext {
    pub fn is_secure(&self) -> bool {
        self.production_keys_loaded && self.ed25519_health_ok && self.blake3_health_ok
    }

    pub fn is_fully_secure(&self) -> bool {
        self.is_secure()
            && self.secure_boot_enabled
            && self.platform_key_verified
            && self.signature_database_valid
    }

    pub fn security_score(&self) -> u32 {
        let mut score: u32 = 0;
        if self.production_keys_loaded {
            score += 20;
        }
        if self.ed25519_health_ok && self.blake3_health_ok {
            score += 20;
        }
        if self.secure_boot_enabled {
            score += 15;
        }
        if self.platform_key_verified {
            score += 10;
        }
        if self.signature_database_valid {
            score += 10;
        }
        if self.hardware_rng_available {
            score += 5;
        }
        if self.measured_boot_active {
            score += 20;
        }
        score
    }
}
