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

use nonos_app_skeleton::PaintBuffer;

use crate::browser::layout::boxmodel::{Content, Fragment};
use crate::browser::state::State;

use super::box_page::TOP;
use super::fill_page::fill_page;
use super::fill_rounded::fill_rounded;

const IMG_BG: u32 = 0xFF20_2A30;
const IMG_EDGE: u32 = 0xFF46_A6B2;

// One display-list rectangle: background, border edges, then content. The
// fragment clip travels in page coordinates and shifts with the scroll.
pub(super) fn box_fragment(
    state: &State,
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    bottom: i32,
) {
    let dy = TOP - state.scroll as i32;
    let clip = f.clip.map(|c| [c[0], c[1].saturating_add(dy), c[2], c[3].saturating_add(dy)]);
    // The drop shadow paints first so the box and its content sit over it.
    if let Some(s) = f.shadow.as_ref() {
        super::shadow::paint_shadow(fb, s, f.x, sy, f.w, f.h);
    }
    if f.bg != 0 {
        fill_rounded(fb, f.x, sy, f.w, f.h, f.radius, f.bg, clip);
    }
    // A decoded background image paints over the color and behind content.
    super::bg_image::paint_bg_image(state, fb, f, sy, bottom, clip);
    // Border edges shorten by the corner radius on each end.
    let r = f.radius as i32;
    let [bt, br, bb, bl] = f.border;
    if bt > 0 {
        fill_page(fb, f.x + r, sy, f.w - 2 * r, bt as i32, f.border_color, clip);
    }
    if bb > 0 {
        fill_page(fb, f.x + r, sy + f.h - bb as i32, f.w - 2 * r, bb as i32, f.border_color, clip);
    }
    if bl > 0 {
        fill_page(fb, f.x, sy + r, bl as i32, f.h - 2 * r, f.border_color, clip);
    }
    if br > 0 {
        fill_page(fb, f.x + f.w - br as i32, sy + r, br as i32, f.h - 2 * r, f.border_color, clip);
    }
    // Text and images draw only when their box sits fully inside the page
    // area and clip; the framebuffer has no clip for glyph or raster runs.
    if sy < TOP || sy + f.h > bottom {
        return;
    }
    if let Some(c) = clip {
        if f.x < c[0] || f.x + f.w > c[2] || sy < c[1] || sy + f.h > c[3] {
            return;
        }
    }
    match &f.content {
        Content::None => {}
        Content::Text { text, color, px, bold, mono, underline, font } => {
            let ty = sy + (f.h - *px as i32).max(0) / 2;
            crate::browser::fonts::draw_text(fb, *font, *mono, f.x, ty, text, *color, *px);
            if *bold {
                crate::browser::fonts::draw_text(fb, *font, *mono, f.x + 1, ty, text, *color, *px);
            }
            if *underline {
                fill_page(fb, f.x, sy + f.h - 2, f.w, 1, *color, clip);
            }
        }
        Content::Image { src, alt, fit } => {
            // The store is keyed by the absolute URL the fetch used, so resolve
            // the fragment's src against the page base before looking it up.
            // Without this every relative image src misses its decoded raster.
            let key = state
                .base
                .as_ref()
                .map(|b| crate::browser::url::join(b, src))
                .unwrap_or_else(|| src.clone());
            if let Some(img) = state.images.ready(&key) {
                crate::browser::image::blit_into(
                    fb,
                    img,
                    f.x.max(0) as u32,
                    sy.max(0) as u32,
                    f.w.max(0) as u32,
                    f.h.max(0) as u32,
                    *fit,
                    clip,
                );
            } else {
                fill_page(fb, f.x, sy, f.w, f.h, IMG_BG, clip);
                fill_page(fb, f.x, sy, f.w, 1, IMG_EDGE, clip);
                fill_page(fb, f.x, sy + f.h - 1, f.w, 1, IMG_EDGE, clip);
                fill_page(fb, f.x, sy, 1, f.h, IMG_EDGE, clip);
                fill_page(fb, f.x + f.w - 1, sy, 1, f.h, IMG_EDGE, clip);
                let label = if alt.is_empty() { "image" } else { alt.as_str() };
                if f.h >= 24 && f.w >= 48 {
                    fb.text_ttf(f.x + 8, sy + 6, label, IMG_EDGE, 14.0);
                }
            }
        }
    }
}
