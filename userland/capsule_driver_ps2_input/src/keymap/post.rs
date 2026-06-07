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
use super::set1::{
    KEYCODE_LALT, KEYCODE_LCTRL, KEYCODE_LMETA, KEYCODE_LSHIFT, KEYCODE_RALT, KEYCODE_RCTRL,
    KEYCODE_RMETA, KEYCODE_RSHIFT,
};
use super::translate::Translated;
use nonos_libc::{mk_input_event_post, InputEvent, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

// Modifier flag bits, matching the app-side MOD_* contract and the USB HID
// driver's encoding (shift, ctrl, alt, meta).
const MOD_SHIFT: u16 = 1 << 0;
const MOD_CTRL: u16 = 1 << 1;
const MOD_ALT: u16 = 1 << 2;
const MOD_META: u16 = 1 << 3;

/// The modifier bit a keycode toggles, or None for a normal key.
pub fn modifier_bit(keycode: u32) -> Option<u16> {
    match keycode {
        KEYCODE_LSHIFT | KEYCODE_RSHIFT => Some(MOD_SHIFT),
        KEYCODE_LCTRL | KEYCODE_RCTRL => Some(MOD_CTRL),
        KEYCODE_LALT | KEYCODE_RALT => Some(MOD_ALT),
        KEYCODE_LMETA | KEYCODE_RMETA => Some(MOD_META),
        _ => None,
    }
}

pub fn publish(t: Translated, mods: u16) -> bool {
    let kind = if t.is_release { INPUT_KIND_KEY_UP } else { INPUT_KIND_KEY_DOWN };
    let ev = InputEvent {
        kind,
        flags: mods,
        code: t.keycode,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    let rc = mk_input_event_post(&ev);
    rc >= 0
}
