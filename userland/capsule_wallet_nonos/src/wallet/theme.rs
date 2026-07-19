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

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 800;
pub const BG: u32 = 0xFF09_0C12;
pub const BG_2: u32 = 0xFF07_0A0F;
pub const LINE: u32 = 0xFF1A_2230;
pub const PANEL: u32 = 0xFF0D_1219;
pub const PANEL_2: u32 = 0xFF0A_0F17;
pub const FG: u32 = 0xFFEC_EFF4;
pub const MUTED: u32 = 0xFF81_8B9A;
pub const ACCENT: u32 = 0xFF3D_D9E8;
pub const CYAN: u32 = 0xFF3D_D9E8;
pub const WARN: u32 = 0xFFEA_B873;

// Dark ink for text on solid cyan/green fills; dim text; stronger border.
pub const INK: u32 = 0xFF04_222A;
pub const DIM: u32 = 0xFF56_5F6E;
pub const LINE2: u32 = 0xFF26_313F;

// Semantic status colors and their dark inks for solid badges.
pub const GREEN: u32 = 0xFF6F_E6A6;
pub const GREEN_INK: u32 = 0xFF05_271A;
pub const AMBER: u32 = 0xFFEA_B873;
pub const AMBER_INK: u32 = 0xFF2B_1E07;

// Sidebar active-row background and the top/bottom system-bar ground.
pub const SEL: u32 = 0xFF13_1A25;
pub const SYSBAR: u32 = 0xFF07_0A0F;

// Compatibility aliases retained while painters migrate.
pub const ELEV_HI: u32 = 0xFF1A_2230;
pub const ELEV_LO: u32 = 0xFF1A_2230;
pub const ACCENT_DIM: u32 = 0xFF13_1A25;
pub const ACCENT_TAG_BG: u32 = 0xFF0F_3B41;
pub const ACCENT_TAG_FG: u32 = 0xFF3D_D9E8;
pub const NEUTRAL_100: u32 = 0xFFEC_EFF4;
pub const NEUTRAL_400: u32 = 0xFFB2_B6CA;
pub const NEUTRAL_800: u32 = 0xFF26_313F;
pub const OK: u32 = 0xFF6F_E6A6;
pub const OK_BG: u32 = 0xFF05_271A;
pub const WARN_BG: u32 = 0xFF2B_1E07;

// Spacing scale.
pub const S1: u32 = 8;
pub const S2: u32 = 16;
pub const S3: u32 = 24;
pub const S4: u32 = 32;
pub const S6: u32 = 48;

// Content geometry: sidebar ends here; screens draw from CONTENT_X inward.
pub const CONTENT_X: u32 = 200;
pub const CONTENT_PAD: u32 = 26;
