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

use nonos_libc::{INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP};

use super::key_event::KeyEvent;
use super::keymap;
use super::post_wire::send;

const KEY_LEFT: u32 = 0xE000;
const KEY_RIGHT: u32 = 0xE001;
const KEY_UP: u32 = 0xE002;
const KEY_DOWN: u32 = 0xE003;
const KEY_HOME: u32 = 0xE004;
const KEY_END: u32 = 0xE005;
const KEY_DELETE: u32 = 0xE006;
const KEY_PAGE_UP: u32 = 0xE007;
const KEY_PAGE_DOWN: u32 = 0xE008;

pub fn publish(ev: KeyEvent) -> bool {
    let kind = if ev.pressed { INPUT_KIND_KEY_DOWN } else { INPUT_KIND_KEY_UP };
    send(kind, flags(ev.modifiers, ev.caps), key_code(ev), 0, 0)
}

fn key_code(ev: KeyEvent) -> u32 {
    match ev.scancode {
        0x4a => KEY_HOME,
        0x4b => KEY_PAGE_UP,
        0x4c => KEY_DELETE,
        0x4d => KEY_END,
        0x4e => KEY_PAGE_DOWN,
        0x4f => KEY_RIGHT,
        0x50 => KEY_LEFT,
        0x51 => KEY_DOWN,
        0x52 => KEY_UP,
        _ => resolved_or_usage(ev),
    }
}

// Printable keys post their layout-resolved codepoint (which may be
// outside ASCII for accented letters); control keys keep the ASCII byte
// from the event; anything else posts its raw usage in the 0x2000 page.
fn resolved_or_usage(ev: KeyEvent) -> u32 {
    let code = keymap::resolve_code(ev.scancode, ev.modifiers, ev.caps);
    if code != 0 {
        return code;
    }
    match ev.ascii {
        0 => 0x2000 | ev.scancode as u32,
        b'\n' => 0x0D,
        c => c as u32,
    }
}

fn flags(modifiers: u8, caps: bool) -> u16 {
    let shift = u16::from((modifiers & 0x22) != 0);
    let ctrl = u16::from((modifiers & 0x11) != 0) << 1;
    let alt = u16::from((modifiers & 0x04) != 0) << 2;
    let altgr = u16::from((modifiers & 0x40) != 0) << 6;
    let meta = u16::from((modifiers & 0x88) != 0) << 3;
    let caps = u16::from(caps) << 4;
    shift | ctrl | alt | altgr | meta | caps
}
