// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::boot::handoff::BootHandoffV1;
use crate::sys::serial;

pub fn log_security_status(handoff: &BootHandoffV1) {
    serial::println(b"[NONOS] === Security Status ===");
    if handoff.meas.kernel_sig_ok != 0 {
        serial::println(b"[NONOS] Kernel signature: VERIFIED");
    } else {
        serial::println(b"[NONOS] Kernel signature: NOT VERIFIED");
    }
    if handoff.meas.secure_boot != 0 {
        serial::println(b"[NONOS] Secure Boot: ENABLED");
    } else {
        serial::println(b"[NONOS] Secure Boot: DISABLED");
    }
    log_entropy(handoff);
    serial::println(b"[NONOS] ========================");
}

fn log_entropy(handoff: &BootHandoffV1) {
    // The seed itself is never printed. It is the bootloader's contribution to
    // the kernel CSPRNG, and a console log of any part of it narrows the search
    // for everything later derived from it. Report only whether it arrived.
    let has_entropy = handoff.rng.seed32.iter().any(|&b| b != 0);
    if has_entropy {
        serial::println(b"[NONOS] RNG seed: AVAILABLE - applying...");
        if let Err(_) = crate::crypto::rng::seed_from_bootloader(&handoff.rng.seed32) {
            serial::println(b"[NONOS] RNG seed: FAILED TO APPLY");
        } else {
            serial::println(b"[NONOS] RNG seed: APPLIED SUCCESSFULLY");
        }
    } else {
        serial::println(b"[NONOS] RNG seed: ALL ZEROS - bootloader entropy missing!");
    }
}
