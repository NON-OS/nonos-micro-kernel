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
pub const BG: u32 = 0xFF0D_1218;
pub const BG_2: u32 = 0xFF10_1821;
pub const LINE: u32 = 0xFF2B_3645;
pub const PANEL: u32 = 0xFF17_202B;
pub const PANEL_2: u32 = 0xFF1E_2A36;
pub const FG: u32 = 0xFFE6_EEF6;
pub const MUTED: u32 = 0xFF8E_9BA9;
pub const ACCENT: u32 = 0xFF65_D17E;
pub const CYAN: u32 = 0xFF76_E4F7;
pub const WARN: u32 = 0xFFE6_C173;

// Elevation edges for card depth (top highlight, bottom shadow).
pub const ELEV_HI: u32 = 0xFF2E_3A48;
pub const ELEV_LO: u32 = 0xFF0A_0E13;
// Dim accent tint for selected rows / badges, keyed off ACCENT.
pub const ACCENT_DIM: u32 = 0xFF16_2A1E;

// 8px spacing scale. Compose layouts from these, not ad-hoc offsets.
pub const S1: u32 = 8;
pub const S2: u32 = 16;
pub const S3: u32 = 24;
pub const S4: u32 = 32;
pub const S6: u32 = 48;

// Content geometry: sidebar ends here; screens draw from CONTENT_X inward.
pub const CONTENT_X: u32 = 336;
pub const CONTENT_PAD: u32 = 32;
