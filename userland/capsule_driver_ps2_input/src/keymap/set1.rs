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
pub const SET1_BASE: [u32; 128] = [
    0,
    0x1B,
    b'1' as u32,
    b'2' as u32,
    b'3' as u32,
    b'4' as u32,
    b'5' as u32,
    b'6' as u32,
    b'7' as u32,
    b'8' as u32,
    b'9' as u32,
    b'0' as u32,
    b'-' as u32,
    b'=' as u32,
    0x08,
    b'\t' as u32,
    b'q' as u32,
    b'w' as u32,
    b'e' as u32,
    b'r' as u32,
    b't' as u32,
    b'y' as u32,
    b'u' as u32,
    b'i' as u32,
    b'o' as u32,
    b'p' as u32,
    b'[' as u32,
    b']' as u32,
    0x0D,
    KEYCODE_LCTRL,
    b'a' as u32,
    b's' as u32,
    b'd' as u32,
    b'f' as u32,
    b'g' as u32,
    b'h' as u32,
    b'j' as u32,
    b'k' as u32,
    b'l' as u32,
    b';' as u32,
    b'\'' as u32,
    b'`' as u32,
    KEYCODE_LSHIFT,
    b'\\' as u32,
    b'z' as u32,
    b'x' as u32,
    b'c' as u32,
    b'v' as u32,
    b'b' as u32,
    b'n' as u32,
    b'm' as u32,
    b',' as u32,
    b'.' as u32,
    b'/' as u32,
    KEYCODE_RSHIFT,
    b'*' as u32,
    KEYCODE_LALT,
    b' ' as u32,
    KEYCODE_CAPS,
    KEYCODE_F1,
    KEYCODE_F2,
    KEYCODE_F3,
    KEYCODE_F4,
    KEYCODE_F5,
    KEYCODE_F6,
    KEYCODE_F7,
    KEYCODE_F8,
    KEYCODE_F9,
    KEYCODE_F10,
    KEYCODE_NUMLK,
    KEYCODE_SCROLL,
    b'7' as u32,
    b'8' as u32,
    b'9' as u32,
    b'-' as u32,
    b'4' as u32,
    b'5' as u32,
    b'6' as u32,
    b'+' as u32,
    b'1' as u32,
    b'2' as u32,
    b'3' as u32,
    b'0' as u32,
    b'.' as u32,
    0,
    0,
    0,
    KEYCODE_F11,
    KEYCODE_F12,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub const KEYCODE_LCTRL: u32 = 0x1001;
pub const KEYCODE_RCTRL: u32 = 0x1002;
pub const KEYCODE_LSHIFT: u32 = 0x1003;
pub const KEYCODE_RSHIFT: u32 = 0x1004;
pub const KEYCODE_LALT: u32 = 0x1005;
pub const KEYCODE_RALT: u32 = 0x1006;
pub const KEYCODE_LMETA: u32 = 0x1007;
pub const KEYCODE_RMETA: u32 = 0x1008;
pub const KEYCODE_CAPS: u32 = 0x1009;
pub const KEYCODE_NUMLK: u32 = 0x100A;
pub const KEYCODE_SCROLL: u32 = 0x100B;
pub const KEYCODE_F1: u32 = 0x1101;
pub const KEYCODE_F2: u32 = 0x1102;
pub const KEYCODE_F3: u32 = 0x1103;
pub const KEYCODE_F4: u32 = 0x1104;
pub const KEYCODE_F5: u32 = 0x1105;
pub const KEYCODE_F6: u32 = 0x1106;
pub const KEYCODE_F7: u32 = 0x1107;
pub const KEYCODE_F8: u32 = 0x1108;
pub const KEYCODE_F9: u32 = 0x1109;
pub const KEYCODE_F10: u32 = 0x110A;
pub const KEYCODE_F11: u32 = 0x110B;
pub const KEYCODE_F12: u32 = 0x110C;
pub const KEYCODE_UP: u32 = 0x1201;
pub const KEYCODE_DOWN: u32 = 0x1202;
pub const KEYCODE_LEFT: u32 = 0x1203;
pub const KEYCODE_RIGHT: u32 = 0x1204;
pub const KEYCODE_HOME: u32 = 0x1205;
pub const KEYCODE_END: u32 = 0x1206;
pub const KEYCODE_PGUP: u32 = 0x1207;
pub const KEYCODE_PGDN: u32 = 0x1208;
pub const KEYCODE_INS: u32 = 0x1209;
pub const KEYCODE_DEL: u32 = 0x120A;