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

//! Switch a Precision Touchpad out of its power-on mouse mode. PTP devices
//! stream the touch collection only after the host writes Input Mode = 3
//! (touchpad) to the device-configuration FEATURE report — Windows and Linux
//! both do this on bind, so a pad that has only ever seen those hosts has
//! never run in mouse mode in the field.

use super::report_desc::TouchLayout;
use crate::i2c_client::write_read;

const OPCODE_GET_REPORT: u8 = 0x02;
const OPCODE_SET_REPORT: u8 = 0x03;
const REPORT_TYPE_FEATURE: u8 = 3;
const PTP_MODE_TOUCHPAD: u8 = 3;
const REPORT_MAX: usize = 62;

/// Write Input Mode = touchpad. Reads the current feature report first so any
/// sibling fields (surface/button switches) keep their values, and falls back
/// to a minimal report when the device refuses the read. Best-effort: returns
/// whether the SET_REPORT write was accepted.
pub fn set_ptp_mode(port: u32, addr: u8, desc: &[u8; 30], layout: &TouchLayout) -> bool {
    if !layout.input_mode.present() || layout.input_mode.bit_offset % 8 != 0 {
        return false;
    }
    let id = layout.input_mode_report_id;
    // Report ids 15 and up need the extended-id command form; PTP pads use
    // small ids, so the simple frame covers the real world.
    if id == 0 || id >= 15 {
        return false;
    }
    let cmd = u16::from_le_bytes([desc[16], desc[17]]);
    let data = u16::from_le_bytes([desc[18], desc[19]]);
    if cmd == 0 || data == 0 {
        return false;
    }
    let c = cmd.to_le_bytes();
    let d = data.to_le_bytes();
    let ty_id = (REPORT_TYPE_FEATURE << 4) | id;

    // Current report content, id byte first. GET_REPORT answers with a 2-byte
    // length prefix covering itself plus the report.
    let mut report = [0u8; REPORT_MAX];
    let mut report_len = 0usize;
    let get = [c[0], c[1], ty_id, OPCODE_GET_REPORT, d[0], d[1]];
    let mut rx = [0u8; 64];
    if let Some(n) = write_read(port, addr, &get, &mut rx) {
        if n >= 3 {
            let total = usize::from(u16::from_le_bytes([rx[0], rx[1]]));
            if (3..=n.min(REPORT_MAX + 2)).contains(&total) {
                report_len = total - 2;
                report[..report_len].copy_from_slice(&rx[2..total]);
            }
        }
    }
    if report_len != 0 && report[0] != id && report_len < REPORT_MAX {
        // Device answered without the leading report-id byte; reinstate it so
        // the offsets and the SET_REPORT payload line up.
        report.copy_within(0..report_len, 1);
        report[0] = id;
        report_len += 1;
    }
    if report_len == 0 {
        report[0] = id;
        report[1] = PTP_MODE_TOUCHPAD;
        report_len = 2;
    } else {
        let off = (layout.input_mode.bit_offset / 8) as usize + 1;
        if off >= report_len {
            return false;
        }
        report[off] = PTP_MODE_TOUCHPAD;
    }

    // SET_REPORT: command register, type/id + opcode, then the data register
    // and the length-prefixed report.
    let len = (2 + report_len) as u16;
    let l = len.to_le_bytes();
    let mut tx = [0u8; 8 + REPORT_MAX];
    tx[0..2].copy_from_slice(&c);
    tx[2] = ty_id;
    tx[3] = OPCODE_SET_REPORT;
    tx[4..6].copy_from_slice(&d);
    tx[6..8].copy_from_slice(&l);
    tx[8..8 + report_len].copy_from_slice(&report[..report_len]);
    write_read(port, addr, &tx[..8 + report_len], &mut []).is_some()
}
