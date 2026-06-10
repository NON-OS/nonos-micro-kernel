// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::rect::Rect;
use super::types::Layout;
use crate::display::gop::get_dimensions;
use crate::fwui::metrics::{line, margin, pad};

pub fn compute() -> Layout {
    let (sw, sh) = get_dimensions();
    let m = margin();
    let frame = Rect::new(m, m, sw.saturating_sub(m * 2), sh.saturating_sub(m * 2));
    let ix = frame.x + pad() * 2;
    let iw = frame.w.saturating_sub(pad() * 4);
    let nav = Rect::new(ix, frame.y + line() * 2, iw, line());
    let content_top = nav.bottom() + line();
    let status_h = line() * 2;
    let status_y = frame.bottom().saturating_sub(line() * 2 + status_h);
    let status = Rect::new(ix, status_y, iw, status_h);
    let content = Rect::new(ix, content_top, iw, status_y.saturating_sub(content_top + line()));
    Layout { screen_w: sw, screen_h: sh, frame, nav, content, status }
}
