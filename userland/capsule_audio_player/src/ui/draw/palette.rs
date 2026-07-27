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

pub const BG: u32 = 0xFF0A0A0F;
pub const PANEL: u32 = 0xFF15151E;
pub const GROOVE: u32 = 0xFF20202C;
pub const ACCENT: u32 = 0xFF8B7FF0;
pub const ACCENT_DIM: u32 = 0xFF6A5FC4;
pub const ACCENT_LIGHT: u32 = 0xFFA99BF7;
pub const WAVE: u32 = 0xFF2B2B3A;
pub const TEXT: u32 = 0xFFF2F1F8;
pub const MUTED: u32 = 0xFF8A8A99;
pub const DIM: u32 = 0xFF5C5C6B;
pub const LINE: u32 = 0xFF19191D;
pub const LINE_2: u32 = 0xFF202025;
pub const ACCENT_WASH: u32 = 0xFF12111D;

pub fn rgb24(argb: u32) -> u32 {
    argb & 0x00FF_FFFF
}
