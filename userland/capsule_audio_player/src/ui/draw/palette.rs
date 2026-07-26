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
pub const GROOVE: u32 = 0xFF232334;
pub const ACCENT: u32 = 0xFF7C5CFF;
pub const TEXT: u32 = 0xFFF2F1F8;
pub const MUTED: u32 = 0xFF8A8A99;

pub fn rgb24(argb: u32) -> u32 {
    argb & 0x00FF_FFFF
}
