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

// Control-strip metrics: the em size every control's label is measured and drawn
// at, a text pill's inner padding, the gap between controls, and the band all of
// them share. Every control in the header sits in this one band.
pub const TOOL_PX: f32 = 17.0;
pub const TOOL_PAD: u32 = 14;
pub const TOOL_GAP: u32 = 10;
pub const TOOL_H: u32 = 32;

// The search field's width, and the window width below which it collapses to a
// bare magnifier so the breadcrumb keeps a usable share of a narrow header.
pub const SEARCH_W: u32 = 160;
pub const FIELD_MIN_W: u32 = 1000;
pub const CLEAR_W: u32 = 26;

// Icon-control metrics: the square box a bare icon button occupies, the glyph
// centred inside it, the gap from a glyph to the label beside it, and the inner
// padding of a pill that leads with an icon rather than with text.
pub const ICON_BTN: u32 = 30;
pub const GLYPH_S: u32 = 16;
pub const GLYPH_GAP: u32 = 6;
pub const ICON_PAD: u32 = 10;

// Breadcrumb metrics. The root is drawn as a home glyph of this width and the
// separators as chevrons centred in a gap of this width.
pub const CRUMB_PX: f32 = 17.0;
pub const CRUMB_PAD: u32 = 10;
pub const CRUMB_SEP_W: u32 = 14;
pub const HOME_W: u32 = 18;
pub const CRUMB_ROOT: &str = "Root";

pub const UNDO_LABEL: &str = "Undo";
pub const NEW_LABEL: &str = "New";
pub const SEARCH_HINT: &str = "Search";

/// What a header click targets. `Crumb` carries the index of the breadcrumb
/// segment, counting the root as 0.
#[derive(Clone, Copy, PartialEq)]
pub enum HeadHit {
    Search,
    SearchClear,
    ViewList,
    ViewGrid,
    Sort,
    Undo,
    New,
    NavBack,
    NavFwd,
    Crumb(usize),
}

/// One laid-out header control: the box it occupies and what clicking it means.
pub struct Slot {
    pub x: u32,
    pub w: u32,
    pub hit: HeadHit,
}
