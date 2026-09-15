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

//! The section-01 colour tokens. No other file may hold a raw ARGB literal.

pub const VOID: u32 = 0xFF04_080D;
pub const DEEP: u32 = 0xFF07_101A;
pub const PANEL: u32 = 0xFF0A_1420;
pub const RAISED: u32 = 0xFF0D_1E2E;
pub const CYAN: u32 = 0xFF35_E2F5;
pub const CYAN_DIM: u32 = 0xFF0E_9DB7;
pub const VIOLET: u32 = 0xFF9B_5CF6;
pub const EDGE: u32 = 0x2435_E2F5;
pub const INK: u32 = 0xFFDC_ECF5;
pub const MID: u32 = 0xFF8F_A9BB;
pub const MUTE: u32 = 0xFF5D_7A8D;
pub const GREEN: u32 = 0xFF3F_D68A;
pub const AMBER: u32 = 0xFFF2_B53C;
pub const RED: u32 = 0xFFFF_6B6B;

pub const SCRIM: u32 = 0x8C00_0000;
pub const HOVER: u32 = 0x1435_E2F5;
pub const PRESS: u32 = 0x2A35_E2F5;
pub const VIOLET_WASH: u32 = 0x309B_5CF6;
pub const CYAN_WASH: u32 = 0x2835_E2F5;

pub fn rgb(argb: u32) -> u32 {
    argb & 0x00FF_FFFF
}

pub fn alpha(rgb: u32, a: u8) -> u32 {
    ((a as u32) << 24) | (rgb & 0x00FF_FFFF)
}

pub fn mix(a: u32, b: u32, t: u32) -> u32 {
    let t = t.min(255);
    let ch = |sh: u32| {
        let x = (a >> sh) & 0xFF;
        let y = (b >> sh) & 0xFF;
        (x * (255 - t) + y * t) / 255
    };
    (ch(16) << 16) | (ch(8) << 8) | ch(0)
}
