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

//! Menu bar press handling: toggle a title, run the row that was clicked, or
//! close on a miss. Rows the capsule cannot perform yet say so in the status bar.

use nonos_app_skeleton::EventOutcome;

use super::app::Editor;
use super::menubar::{menubar_hit, rows, MenuCmd, MenuHit};
use super::on_ctrl::on_ctrl;
use super::unsupported::NO_HANDLER;

impl Editor {
    pub(super) fn menubar_press(&mut self, x: i32, y: i32) -> EventOutcome {
        match menubar_hit(&self.mb_layout, self.mb_open, x, y) {
            MenuHit::Title(i) => {
                self.mb_open = if self.mb_open == Some(i) { None } else { Some(i) };
            }
            MenuHit::Row(r) => {
                let title = self.mb_open.take().unwrap_or(0);
                self.run_menu_row(title, r);
            }
            MenuHit::Outside => self.mb_open = None,
        }
        EventOutcome::Repaint
    }

    fn run_menu_row(&mut self, title: usize, row: usize) {
        let cmd = match rows(title).get(row) {
            Some(&(_, cmd)) => cmd,
            None => return,
        };
        match cmd {
            MenuCmd::Ctrl(code, shift) => {
                let doc = self.doc();
                let _ = on_ctrl(doc, code, shift);
            }
            MenuCmd::NewTab => self.new_tab(),
            MenuCmd::CloseTab => {
                let idx = self.active;
                self.close_tab(idx);
            }
            MenuCmd::ToggleSidebar => {
                self.sidebar_open = !self.sidebar_open;
                if self.sidebar_open {
                    self.tree.reload(self.owner_pid);
                }
            }
            MenuCmd::Info(which) => self.info_tab(which),
            MenuCmd::Todo => self.doc().status = NO_HANDLER,
        }
    }
}
