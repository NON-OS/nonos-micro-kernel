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

//! Drive a touchpad's reporting configuration to the state this device class
//! actually works in. Three feature fields govern whether a Precision
//! Touchpad reports at all and in which shape: Input Mode (0x0D:0x52,
//! 3 = the touch collection, the only collection PTP-only pads generate
//! reports on), the Surface switch (0x0D:0x57, 0 = no motion reports, the
//! pad looks dead), and the Button switch (0x0D:0x58, 0 = no button
//! reports). A previous session, another OS on a warm reboot, or a buggy
//! host can leave any of them off. Every field is located from the report
//! descriptor and applied read-modify-write at bit granularity; when the
//! device refuses GET_REPORT, a descriptor-accurate report is constructed
//! from the parsed field positions instead.

use super::report_desc::{Field, TouchLayout};
use crate::i2c_client::write_read;

const OPCODE_GET_REPORT: u8 = 0x02;
const OPCODE_SET_REPORT: u8 = 0x03;
const REPORT_TYPE_FEATURE: u8 = 3;
/// The Precision Touchpad collection. Modern PTP-only pads implement the
/// mouse collection as a compliance stub that never generates reports;
/// Windows and Linux both switch such pads to mode 3 on bind, so touch
/// mode is the only mode this device class actually reports in. Read
/// pacing comes from the GPIO doorbell.
const INPUT_MODE_TOUCHPAD: u32 = 3;
const SWITCH_ON: u32 = 1;
const REPORT_MAX: usize = 62;

/// Drive input mode to touchpad and both reporting switches on, touching only
/// feature reports that need a change. Returns true when the configuration
/// is known good (already correct, or corrected).
pub fn configure_reporting(port: u32, addr: u8, desc: &[u8; 30], layout: &TouchLayout) -> bool {
    let targets = [
        (layout.input_mode, layout.input_mode_report_id, INPUT_MODE_TOUCHPAD),
        (layout.surface_switch, layout.surface_switch_report_id, SWITCH_ON),
        (layout.button_switch, layout.button_switch_report_id, SWITCH_ON),
    ];
    let mut all_ok = true;
    let mut done_ids = [0u8; 3];
    let mut done = 0usize;
    for (_, id, _) in targets.iter() {
        let id = *id;
        // Ids 15 and up need the extended command form; PTP pads use small
        // ids, so those (and absent fields, id 0) are skipped.
        if id == 0 || id >= 15 || done_ids[..done].contains(&id) {
            continue;
        }
        done_ids[done] = id;
        done += 1;
        all_ok &= configure_one_report(port, addr, desc, id, &targets);
    }
    all_ok
}

// Read-modify-write one feature report: apply every target field that lives
// in it, write back only when a bit actually changed.
fn configure_one_report(
    port: u32,
    addr: u8,
    desc: &[u8; 30],
    id: u8,
    targets: &[(Field, u8, u32); 3],
) -> bool {
    let cmd = u16::from_le_bytes([desc[16], desc[17]]);
    let data = u16::from_le_bytes([desc[18], desc[19]]);
    if cmd == 0 || data == 0 {
        return false;
    }
    let c = cmd.to_le_bytes();
    let d = data.to_le_bytes();
    let ty_id = (REPORT_TYPE_FEATURE << 4) | id;

    // GET_REPORT: 2-byte length prefix, then the report, id byte first.
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
    // When the device refuses GET_REPORT, construct the report from the
    // descriptor instead: every field whose position we parsed is set to its
    // correct value, and the remainder is zero (latency/off-state defaults).
    // This is descriptor-accurate, not a blind guess, and it is the only way
    // to reconfigure a pad whose GET path is unimplemented.
    let mut changed = false;
    if report_len < 2 {
        let mut need_bits = 0u32;
        for (field, field_id, _) in targets.iter() {
            if *field_id == id && field.present() {
                need_bits = need_bits.max(field.bit_offset + field.bit_size);
            }
        }
        if need_bits == 0 {
            return false;
        }
        let body_len = ((need_bits as usize) + 7) / 8;
        if 1 + body_len > REPORT_MAX {
            return false;
        }
        report = [0u8; REPORT_MAX];
        report[0] = id;
        report_len = 1 + body_len;
        changed = true;
    }

    for (field, field_id, value) in targets.iter() {
        if *field_id == id && field.present() {
            changed |=
                set_bits(&mut report[1..report_len], field.bit_offset, field.bit_size, *value);
        }
    }
    if !changed {
        return true;
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

// Write a value into a bit field of a report body. Returns whether any bit
// changed. Out-of-range offsets leave the body untouched.
fn set_bits(body: &mut [u8], bit_offset: u32, bit_size: u32, value: u32) -> bool {
    if bit_size == 0 || bit_size > 32 {
        return false;
    }
    let end = bit_offset as usize + bit_size as usize;
    if end > body.len() * 8 {
        return false;
    }
    let mut changed = false;
    for i in 0..bit_size {
        let bit = ((value >> i) & 1) as u8;
        let at = bit_offset as usize + i as usize;
        let byte = at / 8;
        let mask = 1u8 << (at % 8);
        let current = (body[byte] & mask != 0) as u8;
        if current != bit {
            body[byte] = (body[byte] & !mask) | (bit << (at % 8));
            changed = true;
        }
    }
    changed
}
