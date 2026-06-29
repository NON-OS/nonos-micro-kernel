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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityPolicy {
    Development,
    Standard,
    Hardened,
}

impl SecurityPolicy {
    pub fn from_build() -> Self {
        #[cfg(feature = "hardened")]
        {
            return SecurityPolicy::Hardened;
        }
        #[cfg(all(feature = "standard", not(feature = "hardened")))]
        {
            return SecurityPolicy::Standard;
        }
        #[cfg(all(
            feature = "dev-mode",
            not(feature = "standard"),
            not(feature = "hardened")
        ))]
        {
            return SecurityPolicy::Development;
        }
        #[cfg(not(any(feature = "standard", feature = "hardened", feature = "dev-mode")))]
        {
            compile_error!(
                "nonos-bootloader: no security policy selected. Build with exactly one of \
                 --features production|hardened (real hardware), \
                 --features standard|standard-qemu (emulation with full attestation), or \
                 --features dev-qemu (emulation, checks skipped). \
                 The policy is never defaulted, so a production image cannot ship permissive by accident."
            );
        }
    }

    pub fn level(self) -> u8 {
        match self {
            SecurityPolicy::Development => 0,
            SecurityPolicy::Standard => 1,
            SecurityPolicy::Hardened => 2,
        }
    }

    pub fn stricter(self, other: SecurityPolicy) -> SecurityPolicy {
        if other.level() > self.level() {
            other
        } else {
            self
        }
    }
}
