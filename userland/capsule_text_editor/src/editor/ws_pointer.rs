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

//! Pointer routing for the shell chrome. Returns None when the event belongs
//! to the code pane, so the caller forwards it to the document engine.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::activity_bar::{activity_hit, row_screen};
use super::app::Editor;
use super::layout::{ACTIVITY_W, RIBBON_H, TABBAR_H, TITLEBAR_H};
use super::sb_menu::{menu_hit, open_menu};
use super::screen::Screen;
use super::shell::pane_x;
use super::sidebar::sidebar_row_at;
use super::tabbar::{tab_hit, toolbar_hit};

// The input router posts button codes as bit numbers: 1 = left, 2 = right.
const BTN_RIGHT: u32 = 2;

impl Editor {
    pub(super) fn pointer_event(&mut self, event: &InputEvent) -> Option<EventOutcome> {
        let press = event.kind == InputKind::ButtonDown;
        let (x, y) = (event.x, event.y);
        let px = pane_x(self.sidebar_open) as i32;
        let tb_top = TITLEBAR_H as i32;
        let tb_bot = (TITLEBAR_H + TABBAR_H) as i32;
        let rb_bot = tb_bot + RIBBON_H as i32;

        if press && self.panel.is_some() {
            return Some(self.panel_press(x, y));
        }

        // An open context menu takes the whole next press: run the item under
        // the click, or just close on a miss.
        if press && self.menu.is_some() {
            let menu = self.menu.take();
            if let Some(m) = menu {
                if let Some(action) = menu_hit(&m, x, y) {
                    self.apply_menu_action(action, m.target);
                }
            }
            return Some(EventOutcome::Repaint);
        }

        // The menu bar's dropdown overlays the ribbon, so it is asked first and
        // takes the ribbon's dropdown down with it.
        if press && (self.mb_open.is_some() || y < tb_top) {
            self.rb_open = None;
            return Some(self.menubar_press(x, y));
        }

        let in_ribbon = x >= ACTIVITY_W as i32 && y >= tb_bot && y < rb_bot;
        if press && (self.rb_open.is_some() || in_ribbon) {
            return Some(self.ribbon_press(x, y));
        }

        if press && x >= 0 && (x as u32) < ACTIVITY_W {
            return Some(self.activity_press(activity_hit(y)));
        }

        let in_sidebar = self.sidebar_open && x >= ACTIVITY_W as i32 && x < px && y >= tb_bot;
        if press && in_sidebar {
            let row = sidebar_row_at(y, &self.tree, self.last_h, self.entry.is_some());
            if event.code == BTN_RIGHT {
                self.menu = Some(open_menu(&self.tree, x.max(0) as u32, y.max(0) as u32, row));
            } else if let Some(vi) = row {
                if let Some(path) = self.tree.activate(vi) {
                    self.open_path(&path);
                }
            }
            return Some(EventOutcome::Repaint);
        }

        if press && y >= tb_top && y < tb_bot && x >= px {
            // View controls on the right win over the tab area they sit above.
            if let Some(btn) = toolbar_hit(x, self.last_w) {
                self.toolbar_action(btn);
            } else if let Some((idx, close)) = tab_hit(&self.tab_layout, x) {
                if close {
                    self.close_tab(idx);
                } else {
                    self.active = idx;
                }
            }
            return Some(EventOutcome::Repaint);
        }

        None
    }

    // Apply a view-control click: 0 zoom out, 1 zoom in, 2 cycle the theme. The
    // theme is process-wide, so it is not tied to the active document.
    fn toolbar_action(&mut self, btn: usize) {
        use super::layout::{MAX_SCALE, MIN_SCALE};
        match btn {
            0 => {
                let doc = self.doc();
                doc.font_scale = doc.font_scale.saturating_sub(1).max(MIN_SCALE);
            }
            1 => {
                let doc = self.doc();
                doc.font_scale = (doc.font_scale + 1).min(MAX_SCALE);
            }
            _ => super::theme::cycle(),
        }
    }

    pub(super) fn activity_press(&mut self, row: Option<usize>) -> EventOutcome {
        let Some(row) = row else {
            return EventOutcome::Repaint;
        };
        let target = row_screen(row);
        if self.screen == target {
            if target == Screen::Editor {
                self.sidebar_open = !self.sidebar_open;
                if self.sidebar_open {
                    self.tree.reload(self.owner_pid);
                }
            }
        } else {
            self.screen = target;
        }
        EventOutcome::Repaint
    }
}
