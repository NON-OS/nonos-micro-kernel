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

pub const WIDTH: u32 = 1180;
pub const HEIGHT: u32 = 720;
pub const BG: u32 = 0xFF16_1826;
pub const BG_2: u32 = 0xFF16_1826;
pub const LINE: u32 = 0xFF38_3946;
pub const PANEL: u32 = 0xFF23_2532;
pub const PANEL_2: u32 = 0xFF2E_313F;
pub const FG: u32 = 0xFFE9_E9ED;
pub const MUTED: u32 = 0xFF93_97AB;
pub const ACCENT: u32 = 0xFF3F_D8E8;
pub const CYAN: u32 = 0xFF9A_EAF2;
pub const WARN: u32 = 0xFFE7_B975;

// Card border hairline (design shadow-sm, neutral-800); uniform on all edges.
pub const ELEV_HI: u32 = 0xFF3F_424D;
pub const ELEV_LO: u32 = 0xFF3F_424D;
// Dim accent tint for selected rows, keyed off ACCENT (accent-900).
pub const ACCENT_DIM: u32 = 0xFF10_2F34;

// Design ramp/status tokens for tags and hierarchy.
pub const ACCENT_TAG_BG: u32 = 0xFF16_4E56;
pub const ACCENT_TAG_FG: u32 = 0xFFEA_FDFE;
pub const NEUTRAL_100: u32 = 0xFFF3_F5FE;
pub const NEUTRAL_400: u32 = 0xFFB2_B6CA;
pub const NEUTRAL_800: u32 = 0xFF3F_424D;
pub const OK: u32 = 0xFF6B_DB95;
pub const OK_BG: u32 = 0xFF17_3A29;
pub const WARN_BG: u32 = 0xFF3D_2E14;

// 8px spacing scale. Compose layouts from these, not ad-hoc offsets.
pub const S1: u32 = 8;
pub const S2: u32 = 16;
pub const S3: u32 = 24;
pub const S4: u32 = 32;
pub const S6: u32 = 48;

// Content geometry: sidebar ends here; screens draw from CONTENT_X inward.
pub const CONTENT_X: u32 = 336;
pub const CONTENT_PAD: u32 = 32;
