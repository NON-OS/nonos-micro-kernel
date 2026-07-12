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

// HID usage decoding. Printable keys map to the US base character for
// their physical position and resolve through the shared layout tables
// (nonos_keymap), so USB and PS/2 keyboards agree on every layout, shift
// and caps rule. Control keys keep their ASCII control codes.

const SHIFT_MASK: u8 = 0x22;
const CAPS_LOCK: u8 = 0x39;

pub fn is_caps_lock(scancode: u8) -> bool {
    scancode == CAPS_LOCK
}

// The US base character at a printable HID usage position, 0 otherwise.
fn us_base(scancode: u8) -> u8 {
    match scancode {
        0x04..=0x1d => b'a' + (scancode - 0x04),
        0x1e..=0x26 => b"123456789"[(scancode - 0x1e) as usize],
        0x27 => b'0',
        0x2c => b' ',
        0x2d => b'-',
        0x2e => b'=',
        0x2f => b'[',
        0x30 => b']',
        0x31 => b'\\',
        0x33 => b';',
        0x34 => b'\'',
        0x35 => b'`',
        0x36 => b',',
        0x37 => b'.',
        0x38 => b'/',
        _ => 0,
    }
}

/// Final codepoint for a printable key under the active layout, shift and
/// caps state; 0 for control, navigation and unknown usages.
pub fn resolve_code(scancode: u8, modifiers: u8, caps: bool) -> u32 {
    let base = us_base(scancode);
    if base == 0 {
        return 0;
    }
    let shift = (modifiers & SHIFT_MASK) != 0;
    nonos_keymap::resolve(base as u32, shift, caps, super::active::current())
}

/// ASCII byte for the event queue wire format (one byte on the wire).
/// Printable keys clamp non-ASCII resolutions (accented letters) to 0;
/// the posted input event carries the full codepoint instead.
pub fn ascii(scancode: u8, modifiers: u8, caps: bool) -> u8 {
    match scancode {
        0x28 => return b'\n',
        0x29 => return 0x1b,
        0x2a => return 0x08,
        0x2b => return b'\t',
        _ => {}
    }
    match resolve_code(scancode, modifiers, caps) {
        c @ 0x20..=0x7E => c as u8,
        _ => 0,
    }
}
