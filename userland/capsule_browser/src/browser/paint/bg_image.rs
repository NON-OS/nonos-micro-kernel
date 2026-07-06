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

use crate::browser::layout::boxmodel::Fragment;
use crate::browser::state::State;

use super::box_page::TOP;

// Paint a fragment's background image, scaled to its box and behind content.
// The url resolves against the page base like any other fetched image; a box
// only partly on screen is skipped since the raster path has no clip.
pub(super) fn paint_bg_image(
    state: &State,
    fb: &mut PaintBuffer,
    f: &Fragment,
    sy: i32,
    bottom: i32,
) {
    let Some(src) = f.bg_image.as_deref() else { return };
    if sy < TOP || sy + f.h > bottom {
        return;
    }
    // A gradient renders directly; a url resolves and blits once decoded.
    if super::grad::is_gradient(src) {
        super::grad::paint_gradient(fb, src, f.x, sy, f.w, f.h);
        return;
    }
    let Some(base) = state.base.as_ref() else { return };
    let abs = crate::browser::url::join(base, src);
    if let Some(img) = state.images.ready(&abs) {
        let (bx, by) = (f.x.max(0) as u32, sy.max(0) as u32);
        crate::browser::image::blit_into(
            fb,
            img,
            bx,
            by,
            f.w.max(0) as u32,
            f.h.max(0) as u32,
            crate::browser::css::ObjectFit::Cover,
        );
    }
}
