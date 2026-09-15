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

//! The HID descriptor: thirty bytes at a register the platform names, telling
//! the host where the device's other registers are.

pub const LEN: usize = 30;
pub const VERSION: u16 = 0x0100;

/// The register map a device publishes in its HID descriptor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Registers {
    pub hid_desc: u16,
    pub report_desc: u16,
    pub input: u16,
    pub output: u16,
    pub command: u16,
    pub data: u16,
}

/// The map ELAN and Synaptics touchpads ship with: descriptor at 1, then
/// the others in the order the specification lists them.
pub const REGISTERS: Registers = Registers {
    hid_desc: 0x0001,
    report_desc: 0x0002,
    input: 0x0003,
    output: 0x0004,
    command: 0x0005,
    data: 0x0006,
};

/// Section 5.1.1 of the specification, field by field, little-endian.
pub fn build(
    regs: &Registers,
    report_desc_len: u16,
    max_input: u16,
    vendor: u16,
    product: u16,
) -> [u8; LEN] {
    let fields: [u16; 13] = [
        LEN as u16,
        VERSION,
        report_desc_len,
        regs.report_desc,
        regs.input,
        max_input,
        regs.output,
        0,
        regs.command,
        regs.data,
        vendor,
        product,
        1,
    ];
    let mut out = [0u8; LEN];
    for (i, f) in fields.iter().enumerate() {
        out[2 * i..2 * i + 2].copy_from_slice(&f.to_le_bytes());
    }
    out
}
