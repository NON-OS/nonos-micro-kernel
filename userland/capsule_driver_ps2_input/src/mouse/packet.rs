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
use super::event::MouseEvent;
pub const BUTTON_LEFT: u8 = 1 << 0;
pub const BUTTON_RIGHT: u8 = 1 << 1;
pub const BUTTON_MIDDLE: u8 = 1 << 2;
pub const FLAG_X_OVERFLOW: u8 = 1 << 0;
pub const FLAG_Y_OVERFLOW: u8 = 1 << 1;

/// Decode one movement packet. `bytes` is 3 for a plain mouse or 4 for an
/// IntelliMouse, whose fourth byte carries the scroll wheel.
pub fn parse(bytes: &[u8]) -> Option<MouseEvent> {
    if bytes.len() < 3 {
        return None;
    }
    let b0 = bytes[0];
    if b0 & 0x08 == 0 {
        return None;
    }
    let mut buttons = 0;
    if b0 & 0x01 != 0 {
        buttons |= BUTTON_LEFT;
    }
    if b0 & 0x02 != 0 {
        buttons |= BUTTON_RIGHT;
    }
    if b0 & 0x04 != 0 {
        buttons |= BUTTON_MIDDLE;
    }
    let x_overflow = b0 & 0x40 != 0;
    let y_overflow = b0 & 0x80 != 0;
    let mut flags = 0;
    if x_overflow {
        flags |= FLAG_X_OVERFLOW;
    }
    if y_overflow {
        flags |= FLAG_Y_OVERFLOW;
    }
    // IntelliMouse reports the wheel step as a signed value in the fourth byte.
    let dz = if bytes.len() >= 4 { bytes[3] as i8 } else { 0 };
    Some(MouseEvent {
        dx: axis(bytes[1], b0 & 0x10, x_overflow),
        dy: -axis(bytes[2], b0 & 0x20, y_overflow),
        dz,
        buttons,
        flags,
    })
}

/// Movement on one axis. When the controller reports an overflow the delta byte
/// is the low bits of a value that did not fit, so applying it verbatim makes
/// the cursor leap to a garbage position. Cap it to a bounded step in the
/// reported direction instead, which keeps fast motion smooth and monotonic.
fn axis(v: u8, sign_bit: u8, overflow: bool) -> i16 {
    if overflow {
        if sign_bit != 0 {
            -255
        } else {
            255
        }
    } else {
        sign(v, sign_bit)
    }
}

fn sign(v: u8, sign_bit: u8) -> i16 {
    if sign_bit != 0 {
        v as i16 | -256i16
    } else {
        v as i16
    }
}
