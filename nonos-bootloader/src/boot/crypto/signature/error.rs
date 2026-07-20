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

use uefi::prelude::*;

use crate::display::{
    log_warn, show_error_screen, update_stage, StageStatus, STAGE_ED25519_VERIFY, STAGE_ZK_VERIFY,
};
use crate::log::logger::log_error;
use crate::menu::SecurityMode;

use super::super::super::util::fatal_reset;

pub fn handle_no_signature(st: &mut SystemTable<Boot>, mode: SecurityMode, gop: bool) {
    if mode.requires_signature() {
        log_error("crypto", "kernel has no signature - refusing to boot");
        update_stage(STAGE_ED25519_VERIFY, StageStatus::Failed);
        if gop {
            crate::display::log_error(b"Kernel UNSIGNED");
            show_error_screen(b"Kernel not signed");
        }
        fatal_reset(st, "kernel not signed");
    } else {
        if gop {
            log_warn(b"Ed25519 signature SKIPPED (dev mode)");
        }
        update_stage(STAGE_ED25519_VERIFY, StageStatus::Success);
    }
}

pub fn handle_invalid_signature(st: &mut SystemTable<Boot>, mode: SecurityMode, gop: bool) {
    if mode.requires_signature() {
        log_error("crypto", "kernel signature verification FAILED");
        update_stage(STAGE_ED25519_VERIFY, StageStatus::Failed);
        if gop {
            crate::display::log_error(b"Ed25519 INVALID");
            show_error_screen(b"Signature invalid");
        }
        fatal_reset(st, "kernel signature invalid");
    } else {
        if gop {
            log_warn(b"Ed25519 signature INVALID (dev mode - continuing)");
        }
        update_stage(STAGE_ED25519_VERIFY, StageStatus::Success);
    }
}

/// The kernel signed correctly but its STARK self-attestation did not verify
/// against the enrolled boot root. Under a mode that requires signatures this is
/// a hard refuse: a signature alone is not enough, the kernel must also prove its
/// own measurement. This runs only in stark-kernel-attest builds.
pub fn handle_failed_attestation(st: &mut SystemTable<Boot>, mode: SecurityMode, gop: bool) {
    if mode.requires_signature() {
        log_error("crypto", "kernel STARK self-attestation FAILED - refusing to boot");
        update_stage(STAGE_ZK_VERIFY, StageStatus::Failed);
        if gop {
            crate::display::log_error(b"STARK attestation INVALID");
            show_error_screen(b"Kernel self-attestation invalid");
        }
        fatal_reset(st, "kernel self-attestation invalid");
    } else {
        if gop {
            log_warn(b"STARK self-attestation FAILED (dev mode - continuing)");
        }
        update_stage(STAGE_ZK_VERIFY, StageStatus::Success);
    }
}
