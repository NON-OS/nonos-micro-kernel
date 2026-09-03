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

// The house palette the Settings and Processes restyles established, so About
// reads as one system tool alongside them rather than a third visual language.
// Layout sizes are not here: they live in ui/metrics.rs.
// Grounds: the window, the sidebar, the cards and the chips.
pub const BACKGROUND: u32 = 0xFF0B1319;
pub const SIDEBAR_BG: u32 = 0xFF0A1218;
pub const SIDEBAR_LINE: u32 = 0xFF16262F;
pub const CARD_BG: u32 = 0xFF101C24;
pub const CARD_BORDER: u32 = 0xFF16262F;
pub const STATUS_BG: u32 = 0xFF0D171E;
// Hairlines.
pub const RULE: u32 = 0xFF16262F;
pub const RULE_SOFT: u32 = 0xFF122029;
// Text, brightest to dimmest.
pub const TITLE: u32 = 0xFFEAF4F8;
pub const FOREGROUND: u32 = 0xFFCDDDE5;
pub const LABEL: u32 = 0xFFDBE8EE;
pub const MUTED: u32 = 0xFF6D818C;
// Brand teal and the sidebar states derived from it.
pub const ACCENT: u32 = 0xFF35C4E2;
pub const NAV_FG: u32 = 0xFF93A7B2;
pub const NAV_FG_ACTIVE: u32 = 0xFFA8E7F6;
pub const NAV_BG_ACTIVE: u32 = 0x2035C4E2;
pub const NAV_BORDER_ACTIVE: u32 = 0x5935C4E2;
// Chips and meter tracks.
pub const PILL_BG: u32 = 0xFF14232C;
pub const PILL_BORDER: u32 = 0xFF1D323D;
pub const TRACK_BG: u32 = 0xFF25373F;
// Status hues and their translucent fills. Every tint carries alpha: draw them
// with blend_rect / fill_round only, never fill_rect.
pub const OK: u32 = 0xFF33CF7D;
pub const DANGER: u32 = 0xFFE06C75;
pub const OK_TINT: u32 = 0x2033CF7D;
