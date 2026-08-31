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

//! The open pill's dropdown, painted with the overlays so it sits above the
//! pane. Every row here is backed by the document model, so none draw dimmed.

use nonos_app_skeleton::PaintBuffer;

use super::items::pill_labels;
use super::metrics::{row_h, text_top, RibbonCell, DROP_PAD_X};
use super::panel::panel_rect;
use crate::editor::layout::CHROME_PX;
use crate::editor::theme;

pub(in crate::editor) fn paint_ribbon_drop(
    fb: &mut PaintBuffer,
    cells: &[RibbonCell],
    pill: usize,
) {
    let th = theme::active();
    let (x, y, w, h) = panel_rect(cells, pill);
    fb.fill_rect(x, y, w, h, th.tab_inactive_bg);
    fb.fill_rect(x, y, w, 1, th.line);
    fb.fill_rect(x, y + h - 1, w, 1, th.line);
    fb.fill_rect(x, y, 1, h, th.line);
    fb.fill_rect(x + w - 1, y, 1, h, th.line);

    let rh = row_h();
    let ty = text_top(rh);
    for (i, label) in pill_labels(pill).iter().enumerate() {
        let top = y + 1 + i as u32 * rh;
        let _ = fb.text_ttf(
            (x + DROP_PAD_X) as i32,
            (top + ty) as i32,
            label,
            th.foreground,
            CHROME_PX,
        );
    }
}
