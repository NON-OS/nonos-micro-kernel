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

pub const BACKGROUND: u32 = 0xFF181A1F;
pub const FOREGROUND: u32 = 0xFFCBD0D8;
pub const PROMPT: u32 = 0xFF98C379;
pub const CURSOR: u32 = 0xFF5FB0C9;
pub const ACCENT: u32 = 0xFF5FB0C9;
pub const PATH: u32 = 0xFF7FC9A0;
pub const DIM: u32 = 0xFF707682;
// Chrome (header, tab strip, footer, input bar, command blocks) no longer uses
// fixed shades; it derives from the active `state.bg` via paint::shade::elevate
// so every theme, including the translucent profiles, stays complete.
pub const BLOCK_OK: u32 = 0xFF98C379;
pub const BLOCK_ERR: u32 = 0xFFE06C75;
pub const BLOCK_RUN: u32 = 0xFF707682;
