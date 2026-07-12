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

//! Paint the whole shell each frame: activity bar, sidebar, tab strip, the
//! active document's code pane, and the status bar. The file tree is loaded the
//! first time the shell paints, once the owner pid is known.

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_getpid;

use super::activity_bar::paint_activity;
use super::app::Editor;
use super::paint::paint_document;
use super::sb_entry::paint_entry;
use super::sb_menu::paint_menu;
use super::shell::{pane_rect, pane_x};
use super::sidebar::paint_sidebar;
use super::statusbar::paint_status;
use super::tabbar::paint_tabs;
use super::theme;

impl Editor {
    pub(super) fn paint_shell(&mut self, fb: &mut PaintBuffer) {
        if self.owner_pid == 0 {
            self.owner_pid = mk_getpid();
        }
        if !self.tree.loaded {
            self.tree.reload(self.owner_pid);
        }

        let (w, h) = (fb.width, fb.height);
        self.last_w = w;
        self.last_h = h;
        fb.fill_rect(0, 0, w, h, theme::active().background);

        paint_activity(fb, h, self.sidebar_open);
        if self.sidebar_open {
            paint_sidebar(fb, &self.tree, h, self.entry.is_some());
        }
        let px = pane_x(self.sidebar_open);
        self.tab_layout = paint_tabs(fb, &self.docs, self.active, px, w);

        let (rx, ry, rw, rh) = pane_rect(w, h, self.sidebar_open);
        let i = self.active.min(self.docs.len().saturating_sub(1));
        {
            let d = &mut self.docs[i];
            d.pane_x = rx;
            d.pane_y = ry;
            d.pane_w = rw;
            d.pane_h = rh;
            paint_document(d, fb);
        }
        paint_status(fb, &self.docs[i], w, h);

        // Overlays last so they sit above everything else.
        if self.sidebar_open {
            if let Some(entry) = &self.entry {
                paint_entry(fb, entry);
            }
        }
        if let Some(menu) = &self.menu {
            paint_menu(fb, menu);
        }
    }
}
