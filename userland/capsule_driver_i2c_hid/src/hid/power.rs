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

//! I2C-HID power-up handshake. Real devices ship asleep: until the host
//! writes SET_POWER(ON) and RESET through the command register, most
//! touchpads never produce a single input report. The emulated bench has
//! no i2c-hid device, so only real hardware exercises this path.

use nonos_libc::mk_yield;

use crate::i2c_client::write_read;

const OPCODE_RESET: u8 = 0x01;
const OPCODE_SET_POWER: u8 = 0x08;
const POWER_ON: u8 = 0x00;

// A device may take tens of milliseconds to come out of RESET before it
// raises the zero-length reset sentinel on the input register. Poll that
// register with yields between reads, over a budget long enough to cover the
// ~60ms a slow touchpad needs, rather than a single blind settle.
const RESET_POLL_ATTEMPTS: u32 = 64;
const RESET_POLL_YIELDS: u32 = 2048;

/// wCommandRegister from the 30-byte HID descriptor.
pub fn command_register(desc: &[u8; 30]) -> u16 {
    u16::from_le_bytes([desc[16], desc[17]])
}

/// The I2C-HID power-up sequence. Wire framing per the HID-over-I2C spec: after
/// the two command-register bytes, the THIRD byte carries the report id (or
/// power state) in its low nibble and the report type in the high nibble, and
/// the OPCODE goes in the FOURTH byte — RESET is [reg, 0x00, 0x01] and
/// SET_POWER(ON) is [reg, 0x00, 0x08]. The previous framing put the opcode in
/// the third byte, which a conforming device decodes as a reserved command and
/// ignores, making the whole handshake a silent no-op (the monolithic driver
/// had the same bug and only worked because the pad already streamed in its
/// power-on default mode). Order follows the spec and Linux i2c-hid: power on
/// first, settle (ELAN pads need it before they accept commands), then RESET
/// and wait for the reset sentinel. SET_IDLE is not sent: its short 4-byte form
/// is nonstandard and Linux omits it for touchpads. Returns false when the
/// command register is absent or a write is rejected; the caller keeps polling
/// regardless, which still serves devices that wake without the handshake.
pub fn wake(port: u32, addr: u8, desc: &[u8; 30], input_reg: u16) -> bool {
    let cmd_reg = command_register(desc);
    if cmd_reg == 0 {
        return false;
    }
    let reg = cmd_reg.to_le_bytes();
    let set_power = [reg[0], reg[1], POWER_ON, OPCODE_SET_POWER];
    if write_read(port, addr, &set_power, &mut []).is_none() {
        return false;
    }
    settle();
    let reset = [reg[0], reg[1], 0x00, OPCODE_RESET];
    if write_read(port, addr, &reset, &mut []).is_none() {
        return false;
    }
    // The device answers RESET with a zero-length input report on the input
    // register. Wait for that sentinel with a bounded, yield-spaced retry loop
    // and consume it so it is not parsed as input.
    await_reset(port, addr, input_reg)
}

// Read the input register until the zero-length reset report appears, bounding
// the total wait so a missing device cannot hang the driver. Returns true when
// the reset sentinel was observed.
fn await_reset(port: u32, addr: u8, input_reg: u16) -> bool {
    if input_reg == 0 {
        settle();
        return false;
    }
    let reg = input_reg.to_le_bytes();
    for _ in 0..RESET_POLL_ATTEMPTS {
        let mut drain = [0u8; 2];
        // Spec first: after reset the device auto-points at the input register
        // and the sentinel comes from a bare read. Devices that only answer a
        // register-addressed read get the fallback.
        let n = match write_read(port, addr, &[], &mut drain) {
            Some(n) if n >= 2 => Some(n),
            _ => write_read(port, addr, &reg, &mut drain),
        };
        if let Some(n) = n {
            // A zero-length report is encoded as a 0x0000 length prefix.
            if n >= 2 && u16::from_le_bytes([drain[0], drain[1]]) == 0 {
                return true;
            }
        }
        for _ in 0..RESET_POLL_YIELDS {
            mk_yield();
        }
    }
    false
}

fn settle() {
    for _ in 0..2048 {
        mk_yield();
    }
}
