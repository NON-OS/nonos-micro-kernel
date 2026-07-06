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

use crate::browser::css::color::parse_color;
use crate::browser::css::computed::{Computed, TextAlign, TextTransform, WhiteSpace};
use crate::browser::css::parse_line_height::parse_line_height;
use crate::browser::css::parse_px::parse_px;

const MAX_FONT_PX: u32 = 96;

// Text properties: color, weight, family, size, line height and alignment.
pub(super) fn apply_text(
    c: &mut Computed,
    name: &str,
    value: &str,
    fs: u32,
    parent_fs: u32,
) -> bool {
    match name {
        "color" => {
            if let Some(rgb) = parse_color(value) {
                c.color = rgb;
            }
        }
        "font-weight" => match value.trim() {
            "normal" | "100" | "200" | "300" | "400" => c.bold = false,
            "bold" | "bolder" | "500" | "600" | "700" | "800" | "900" => c.bold = true,
            _ => {}
        },
        // The monospace generic (or an explicit mono family) switches text to
        // the fixed-pitch face. A named first family keys the custom face the
        // text draws in once its @font-face loads; a generic keeps the
        // built-in one.
        "font-family" => {
            c.mono = value.to_ascii_lowercase().contains("mono");
            let first = value.split(',').next().unwrap_or("").trim();
            let bare = first.trim_matches('"').trim_matches('\'').trim();
            c.font_key = match bare.to_ascii_lowercase().as_str() {
                "" | "sans-serif" | "serif" | "system-ui" | "ui-sans-serif" => 0,
                "monospace" | "ui-monospace" | "cursive" | "fantasy" => 0,
                _ => crate::browser::fonts::family_key(bare),
            };
        }
        // Em resolves against the parent font size. Clamp above zero so a
        // styled element never reads as "no CSS size" downstream.
        "font-size" => {
            if let Some(px) = parse_px(value, parent_fs) {
                c.font_size_px = px.clamp(1, MAX_FONT_PX);
            }
        }
        "line-height" => {
            if let Some(px) = parse_line_height(value, fs) {
                c.line_height_px = px.min(4 * MAX_FONT_PX);
            }
        }
        "text-decoration" | "text-decoration-line" => {
            c.underline = value.split_whitespace().any(|t| t == "underline");
        }
        "white-space" => match value.trim() {
            "pre" | "pre-wrap" | "pre-line" | "break-spaces" => c.white_space = WhiteSpace::Pre,
            "nowrap" => c.white_space = WhiteSpace::Nowrap,
            "normal" => c.white_space = WhiteSpace::Normal,
            _ => {}
        },
        "text-transform" => match value.trim() {
            "uppercase" => c.text_transform = TextTransform::Upper,
            "lowercase" => c.text_transform = TextTransform::Lower,
            "capitalize" => c.text_transform = TextTransform::Capitalize,
            "none" => c.text_transform = TextTransform::None,
            _ => {}
        },
        "text-align" => match value.trim() {
            "left" | "start" => c.text_align = TextAlign::Left,
            "center" => c.text_align = TextAlign::Center,
            "right" | "end" => c.text_align = TextAlign::Right,
            _ => {}
        },
        _ => return false,
    }
    true
}
