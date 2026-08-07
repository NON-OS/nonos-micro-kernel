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

pub const DEFAULT_FG: u32 = 0xFF1A_1A1A;
pub const DEFAULT_FONT_PX: u32 = 16;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Auto,
    Px(u32),
    Pct(u16),
    // calc(): fixed pixels plus per-mille of the containing base, either
    // part possibly negative.
    Calc(i32, i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WhiteSpace {
    // Collapse runs of whitespace and wrap at the content edge.
    Normal,
    // Preserve spaces and newlines; wrap only at a newline.
    Pre,
    // Collapse whitespace but never wrap.
    Nowrap,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjectFit {
    // Fit the whole image inside the box, letterboxing the spare space.
    Contain,
    // Fill the box, cropping whatever overflows after covering it.
    Cover,
    // Stretch the image to the box, ignoring its aspect ratio.
    Fill,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextTransform {
    None,
    Upper,
    Lower,
    Capitalize,
}

// background-size for the box's image layer. Auto keeps the natural size and
// tiles per background-repeat; a length scales the tile width keeping aspect.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BgSize {
    Auto,
    Cover,
    Contain,
    Px(u16),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    Between,
    Around,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Static,
    Relative,
    Absolute,
}

/// Whether a box is taken out of the normal flow to one side, with the rest of
/// the block's content wrapping around it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Float {
    None,
    Left,
    Right,
}

/// Which floats a box drops below before it is laid: `clear`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Clear {
    None,
    Left,
    Right,
    Both,
}

// One grid column track: a length, a percentage of the row, or a fraction
// of the leftover space.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridTrack {
    Px(u32),
    Pct(u16),
    Fr(u16),
}

pub const MAX_GRID_COLS: usize = 8;

// repeat(auto-fill | auto-fit, ...): how many times the track repeats depends
// on the container width, which the cascade does not know, so it records the
// keyword and layout resolves the count. auto-fit additionally drops the
// tracks no item lands in, so the occupied ones share the whole width.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AutoRepeat {
    Fill,
    Fit,
}

// Inherited fields: color, bold, font_size_px, text_align, line_height_px.
// Everything else is per-element; the cascade walk starts each element from
// root() and copies only the inherited fields across.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Shadow {
    pub dx: i32,
    pub dy: i32,
    pub blur: u32,
    pub color: u32,
}

#[derive(Clone, Copy)]
pub struct Computed {
    pub display_none: bool,
    pub color: u32,
    pub bg: u32,
    pub bold: bool,
    pub mono: bool,
    pub font_size_px: u32,
    pub text_align: TextAlign,
    pub white_space: WhiteSpace,
    pub object_fit: ObjectFit,
    pub text_transform: TextTransform,
    // Family key of the custom face this text draws in, 0 for the built-in
    // face; the fonts registry maps keys to loaded faces.
    pub font_key: u32,
    // Text draws in a webfont icon family (Material Icons/Symbols, Font
    // Awesome) whose face we do not load. Ligature names like "arrow_forward"
    // are mapped to a Unicode glyph so the icon reads as a symbol, not a word.
    pub icon_font: bool,
    pub bg_size: BgSize,
    pub bg_repeat: bool,
    // Element opacity 0..255; multiplies down the subtree at layout.
    pub opacity: u8,
    // Extra advance in px between glyphs, letter-spacing; inherited, may be
    // negative for tightened headings.
    pub letter_spacing: f32,
    pub underline: bool,
    // 0 means unset: derive 1.3 * font size where a line height is needed.
    pub line_height_px: u32,
    pub margin_top: u32,
    pub margin_right: u32,
    pub margin_bottom: u32,
    pub margin_left: u32,
    pub pad_top: u32,
    pub pad_right: u32,
    pub pad_bottom: u32,
    pub pad_left: u32,
    pub border_top: u32,
    pub border_right: u32,
    pub border_bottom: u32,
    pub border_left: u32,
    // 0 means unset: borders fall back to the text color.
    pub border_color: u32,
    pub width: Size,
    pub max_width: Size,
    pub min_width: Size,
    pub height: Size,
    pub min_height: Size,
    pub max_height: Size,
    // Set when both horizontal margins are auto: an in-flow block centres in
    // its available width. The per-side flags track partial declarations.
    pub margin_auto_x: bool,
    pub margin_left_auto: bool,
    pub margin_right_auto: bool,
    // box-sizing: border-box makes width/height span the border box; the
    // default content-box adds padding and border on top.
    pub border_box: bool,
    pub is_block: bool,
    pub is_flex: bool,
    pub is_inline_block: bool,
    // Table roles, set from the tag as a user-agent default or `display: table*`.
    // A table box lays its rows and cells into a shared column grid.
    pub is_table: bool,
    pub is_table_row: bool,
    pub is_table_cell: bool,
    pub flex_wrap: bool,
    pub flex_col: bool,
    pub justify: Justify,
    pub align: Align,
    pub gap: u32,
    pub flex_grow: u32,
    // flex-basis: the main-size base before grow/shrink. Auto means size to
    // content or the width property, as CSS specifies.
    pub flex_basis: Size,
    pub position: Position,
    // float takes the box to one side and flows following content around it;
    // clear drops a box below the floats named by it.
    pub float: Float,
    pub clear: Clear,
    // position: fixed is laid out like absolute but painted without the
    // scroll offset, so it pins to the viewport.
    pub is_fixed: bool,
    // position: sticky flows normally and clamps against the viewport top
    // once scrolled past its threshold.
    pub is_sticky: bool,
    pub top: Size,
    pub right: Size,
    pub bottom: Size,
    pub left: Size,
    pub overflow_hidden: bool,
    pub z: i32,
    pub radius: u32,
    pub shadow: Option<Shadow>,
    pub is_grid: bool,
    // display: contents drops the box and promotes the children.
    pub is_contents: bool,
    pub grid_cols: [GridTrack; MAX_GRID_COLS],
    pub grid_col_n: u8,
    // Set when the template is a single repeat(auto-fill | auto-fit, track);
    // grid_cols then holds that one track and grid_auto_min its floor.
    pub grid_auto: Option<AutoRepeat>,
    pub grid_auto_min: GridTrack,
    // list-style-type: none suppresses the marker on li boxes; inherited.
    pub list_none: bool,
}

impl Computed {
    pub fn root() -> Self {
        Computed {
            display_none: false,
            color: DEFAULT_FG,
            bg: 0,
            bold: false,
            mono: false,
            font_size_px: DEFAULT_FONT_PX,
            text_align: TextAlign::Left,
            white_space: WhiteSpace::Normal,
            object_fit: ObjectFit::Contain,
            text_transform: TextTransform::None,
            font_key: 0,
            icon_font: false,
            bg_size: BgSize::Auto,
            bg_repeat: true,
            opacity: 255,
            letter_spacing: 0.0,
            underline: false,
            line_height_px: 0,
            margin_top: 0,
            margin_right: 0,
            margin_bottom: 0,
            margin_left: 0,
            pad_top: 0,
            pad_right: 0,
            pad_bottom: 0,
            pad_left: 0,
            border_top: 0,
            border_right: 0,
            border_bottom: 0,
            border_left: 0,
            border_color: 0,
            width: Size::Auto,
            max_width: Size::Auto,
            min_width: Size::Auto,
            height: Size::Auto,
            min_height: Size::Auto,
            max_height: Size::Auto,
            margin_auto_x: false,
            margin_left_auto: false,
            margin_right_auto: false,
            border_box: false,
            is_block: false,
            is_flex: false,
            is_inline_block: false,
            is_table: false,
            is_table_row: false,
            is_table_cell: false,
            flex_wrap: false,
            flex_col: false,
            justify: Justify::Start,
            align: Align::Stretch,
            gap: 0,
            flex_grow: 0,
            flex_basis: Size::Auto,
            position: Position::Static,
            float: Float::None,
            clear: Clear::None,
            is_fixed: false,
            is_sticky: false,
            top: Size::Auto,
            right: Size::Auto,
            bottom: Size::Auto,
            left: Size::Auto,
            overflow_hidden: false,
            z: 0,
            radius: 0,
            shadow: None,
            is_grid: false,
            is_contents: false,
            grid_cols: [GridTrack::Fr(1); MAX_GRID_COLS],
            grid_col_n: 0,
            grid_auto: None,
            grid_auto_min: GridTrack::Px(0),
            list_none: false,
        }
    }

    // Fresh element style: defaults for everything, inherited fields carried
    // over from the parent.
    pub fn inherit_from(parent: &Computed) -> Self {
        let mut c = Computed::root();
        c.color = parent.color;
        c.bold = parent.bold;
        c.mono = parent.mono;
        c.font_size_px = parent.font_size_px;
        c.text_align = parent.text_align;
        c.white_space = parent.white_space;
        c.underline = parent.underline;
        c.line_height_px = parent.line_height_px;
        c.list_none = parent.list_none;
        c.text_transform = parent.text_transform;
        c.font_key = parent.font_key;
        c.icon_font = parent.icon_font;
        c.letter_spacing = parent.letter_spacing;
        c
    }

    // Line height in px, deriving the CSS "normal" ratio when unset.
    pub fn line_height(&self) -> u32 {
        if self.line_height_px != 0 {
            self.line_height_px
        } else {
            (self.font_size_px * 13 + 5) / 10
        }
    }
}
