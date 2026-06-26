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

use super::theme::STATUS;
use crate::display::font::{draw_string, CHAR_WIDTH};
use crate::security::SecurityContext;

pub(super) fn draw_security_status(w: u32, y: u32, sec: &SecurityContext) {
    let mut buf = [0u8; 64];
    let mut n = 0usize;
    let head: &[u8] = if sec.measured_boot_active {
        b"TPM MEASURED BOOT ACTIVE   NV COUNTER "
    } else {
        b"TPM MEASURED BOOT INACTIVE   NV COUNTER "
    };
    for &b in head {
        buf[n] = b;
        n += 1;
    }
    let counter: &[u8] = if sec.tpm_counter_ok { b"OK" } else { b"--" };
    for &b in counter {
        buf[n] = b;
        n += 1;
    }
    let msg = &buf[..n];
    let mw = msg.len() as u32 * CHAR_WIDTH;
    draw_string(w.saturating_sub(mw) / 2, y, msg, STATUS);
}
