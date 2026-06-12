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

use crate::handoff::types::{flags, CryptoHandoff};

/// Build handoff flags bitmask from detected hardware and security features.
pub fn build_handoff_flags(
    fb_available: bool,
    acpi_available: bool,
    crypto: &CryptoHandoff,
    tpm_measured: bool,
    smep: bool,
    smap: bool,
    umip: bool,
) -> u64 {
    let mut f: u64 = 0;
    if fb_available {
        f |= flags::FB_AVAILABLE;
    }
    if acpi_available {
        f |= flags::ACPI_AVAILABLE;
    }
    if crypto.secure_boot {
        f |= flags::SECURE_BOOT;
    }
    if crypto.zk_attested {
        f |= flags::ZK_ATTESTED;
    }
    if tpm_measured {
        f |= flags::TPM_MEASURED;
    }
    if smep {
        f |= flags::SMEP;
    }
    if smap {
        f |= flags::SMAP;
    }
    if umip {
        f |= flags::UMIP;
    }
    f |= flags::WX;
    f |= flags::NXE;
    f |= flags::IDMAP_PRESERVED;
    f
}
