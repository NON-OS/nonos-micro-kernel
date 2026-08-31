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

// Every chrome colour in the capsule comes from here. Grounds first: the
// window, the panels, the board and the rail.
pub const BACKGROUND: u32 = 0xFF0B1319;
pub const PANEL_BG: u32 = 0xFF101C24;
pub const PANEL_BORDER: u32 = 0xFF16262F;
pub const BOARD_BG: u32 = 0xFF0D171E;
pub const RAIL_BG: u32 = 0xFF0A1218;
pub const SCRIM: u32 = 0xC00B1319;
// Hairlines, the dotted board grid and row banding.
pub const RULE: u32 = 0xFF16262F;
pub const RULE_SOFT: u32 = 0xFF122029;
pub const GRID_DOT: u32 = 0x14FFFFFF;
pub const BAND: u32 = 0xFF101A21;
// Text, brightest to dimmest.
pub const TITLE: u32 = 0xFFEAF4F8;
pub const FOREGROUND: u32 = 0xFFCDDDE5;
pub const LABEL: u32 = 0xFFDBE8EE;
pub const MUTED: u32 = 0xFF6D818C;
// Brand teal, carried over from the house palette for chrome and selection.
pub const ACCENT: u32 = 0xFF35C4E2;
pub const ACCENT_TINT: u32 = 0x2035C4E2;
pub const ACCENT_BORDER: u32 = 0x5935C4E2;
// Buttons and toggles.
pub const BTN_BG: u32 = 0xFF14232C;
pub const BTN_BORDER: u32 = 0xFF1D323D;
pub const BTN_HOVER_BG: u32 = 0x14FFFFFF;
pub const TRACK_BG: u32 = 0xFF25373F;
// The snake reads head-to-tail as a gradient between these two.
pub const SNAKE_HEAD: u32 = 0xFF7DF9A6;
pub const SNAKE_TAIL: u32 = 0xFF2E8B57;
// Glow is three concentric strokes at falling alpha, not a blur pass.
pub const HALO_NEAR: u32 = 0x707DF9A6;
pub const HALO_MID: u32 = 0x387DF9A6;
pub const HALO_FAR: u32 = 0x187DF9A6;
// Board pieces.
pub const FOOD: u32 = 0xFFE0533D;
pub const FOOD_RING: u32 = 0x66E0533D;
pub const POWER: u32 = 0xFFB482F0;
pub const POWER_RING: u32 = 0x66B482F0;
pub const WALL: u32 = 0xFF2A3F4B;
pub const WALL_EDGE: u32 = 0xFF3A5464;
// Status hues and their translucent fills.
pub const OK: u32 = 0xFF33CF7D;
pub const AMBER: u32 = 0xFFE0A44A;
pub const DANGER: u32 = 0xFFE06C75;
pub const OK_TINT: u32 = 0x2033CF7D;
pub const AMBER_TINT: u32 = 0x22E0A44A;
pub const DANGER_TINT: u32 = 0x22E06C75;
