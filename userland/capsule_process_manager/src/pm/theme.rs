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

// The house palette the Settings restyle established, so the monitor reads as
// one system tool alongside it rather than a second visual language.
// Grounds: the window, the sidebar, the cards and the table body.
pub const BACKGROUND: u32 = 0xFF0B1319;
pub const SIDEBAR_BG: u32 = 0xFF0A1218;
pub const SIDEBAR_LINE: u32 = 0xFF16262F;
pub const CARD_BG: u32 = 0xFF101C24;
pub const CARD_BORDER: u32 = 0xFF16262F;
pub const TABLE_BG: u32 = 0xFF0D171E;
pub const HEADER_BG: u32 = 0xFF0F1A21;
// Hairlines and zebra banding.
pub const RULE: u32 = 0xFF16262F;
pub const RULE_SOFT: u32 = 0xFF122029;
pub const BAND: u32 = 0xFF101A21;
// Text, brightest to dimmest.
pub const TITLE: u32 = 0xFFEAF4F8;
pub const FOREGROUND: u32 = 0xFFCDDDE5;
pub const LABEL: u32 = 0xFFDBE8EE;
pub const WARNING: u32 = 0xFF93A7B2;
pub const MUTED: u32 = 0xFF6D818C;
// Brand teal and the sidebar states derived from it.
pub const ACCENT: u32 = 0xFF35C4E2;
pub const NAV_FG: u32 = 0xFF93A7B2;
pub const NAV_FG_ACTIVE: u32 = 0xFFA8E7F6;
pub const NAV_BG_ACTIVE: u32 = 0x2035C4E2;
pub const NAV_BORDER_ACTIVE: u32 = 0x5935C4E2;
// Row washes. These carry alpha: draw them with blend_rect / fill_round only.
pub const SELECT_BG: u32 = 0x1C35C4E2;
pub const ROW_HOVER_BG: u32 = 0x14FFFFFF;
// Chips, meter tracks and the search field.
pub const PILL_BG: u32 = 0xFF14232C;
pub const PILL_BORDER: u32 = 0xFF1D323D;
pub const TRACK_BG: u32 = 0xFF25373F;
pub const SEARCH_BG: u32 = 0xFF16232B;
pub const SEARCH_BORDER: u32 = 0xFF24363F;
// Status hues and their translucent fills.
pub const OK: u32 = 0xFF33CF7D;
pub const AMBER: u32 = 0xFFE0A44A;
pub const DANGER: u32 = 0xFFE06C75;
pub const OK_TINT: u32 = 0x2033CF7D;
pub const AMBER_TINT: u32 = 0x22E0A44A;
pub const DANGER_TINT: u32 = 0x22E06C75;
pub const ACCENT_TINT: u32 = 0x2035C4E2;
