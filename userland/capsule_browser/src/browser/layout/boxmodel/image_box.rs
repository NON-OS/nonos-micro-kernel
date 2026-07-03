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

use crate::browser::css::Computed;

const DEFAULT_IMG_W: i32 = 320;

// Box for an <img>: CSS sizes win; otherwise a capped default width with a
// 4:3 height. Natural dimensions arrive after decode and cannot drive layout.
pub(super) fn image_box(s: &Computed, content_w: i32) -> (i32, i32) {
    let max_w = content_w.max(1);
    let w = s.width.resolve(content_w).unwrap_or_else(|| content_w.min(DEFAULT_IMG_W));
    let w = w.clamp(1, max_w);
    let h = match s.height.definite_px() {
        Some(p) => p.max(1),
        None => (w * 3 / 4).max(1),
    };
    (w, h)
}
