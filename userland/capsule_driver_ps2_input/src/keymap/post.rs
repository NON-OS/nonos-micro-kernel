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
use super::active;
use super::set1::{
    KEYCODE_LALT, KEYCODE_LCTRL, KEYCODE_LMETA, KEYCODE_LSHIFT, KEYCODE_RALT, KEYCODE_RCTRL,
    KEYCODE_RMETA, KEYCODE_RSHIFT,
};
use super::translate::Translated;
use nonos_libc::{mk_input_event_post, InputEvent, INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

// Modifier flag bits, matching the app-side MOD_* contract and the USB HID
// driver's encoding (shift, ctrl, alt, meta, caps).
pub const MOD_SHIFT: u16 = 1 << 0;
pub const MOD_CTRL: u16 = 1 << 1;
pub const MOD_ALT: u16 = 1 << 2;
pub const MOD_META: u16 = 1 << 3;
pub const MOD_CAPS: u16 = 1 << 4;
pub const MOD_ALTGR: u16 = 1 << 6;

/// The modifier bit a keycode toggles, or None for a normal key.
pub fn modifier_bit(keycode: u32) -> Option<u16> {
    match keycode {
        KEYCODE_LSHIFT | KEYCODE_RSHIFT => Some(MOD_SHIFT),
        KEYCODE_LCTRL | KEYCODE_RCTRL => Some(MOD_CTRL),
        KEYCODE_LALT => Some(MOD_ALT),
        // Right alt is AltGr on every layout here but US, where nothing is
        // mapped to it, so it never doubles as alt.
        KEYCODE_RALT => Some(MOD_ALTGR),
        KEYCODE_LMETA | KEYCODE_RMETA => Some(MOD_META),
        _ => None,
    }
}

pub fn publish(t: Translated, mods: u16, caps: bool) -> bool {
    let kind = if t.is_release { INPUT_KIND_KEY_UP } else { INPUT_KIND_KEY_DOWN };
    // Printable keys carry their final character: the US base from the
    // scancode table resolved through the active layout, shift and caps
    // state, so shift-minus arrives as '_' and shift-a as 'A'. Navigation,
    // function and modifier keycodes live above the ASCII range and pass
    // through. Key-up events resolve with the modifiers held at release;
    // character input only reads key-down, so the asymmetry is harmless.
    let code = nonos_keymap::resolve(
        t.keycode,
        mods & MOD_SHIFT != 0,
        caps,
        mods & MOD_ALTGR != 0,
        active::current(),
    );
    // A key the active layout leaves empty, such as the ISO key on a US
    // layout, produces nothing rather than a null character.
    if code == 0 {
        return false;
    }
    let flags = if caps { mods | MOD_CAPS } else { mods };
    let ev = InputEvent { kind, flags, code, x: 0, y: 0, delta_x: 0, delta_y: 0, timestamp_ns: 0 };
    let rc = mk_input_event_post(&ev);
    rc >= 0
}
