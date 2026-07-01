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

use crate::browser::manifest::WIDTH;

pub const PAGE_BG: u32 = 0xFF18_1B20;
pub const PILL_BG: u32 = 0xFF20_242C;
pub const FG: u32 = 0xFFE8_EAED;
pub const ACCENT: u32 = 0xFF8A_B4F8;
pub const BORDER: u32 = 0xFF3A_3F4B;
pub const DIM: u32 = 0xFF9A_A0A6;
pub const WHITE: u32 = 0xFFFF_FFFF;
pub const CONTENT_TOP: u32 = 80;
pub const PILL_W: u32 = 640;
pub const PILL_H: u32 = 46;
pub const PILL_X: u32 = (WIDTH - PILL_W) / 2;
pub const PILL_Y: u32 = 170;
pub const COUNT: u32 = 4;
pub const CELL_W: u32 = 150;
pub const ROW_X0: u32 = (WIDTH - COUNT * CELL_W) / 2;
pub const BADGE: u32 = 56;
pub const BADGE_Y: u32 = 300;
