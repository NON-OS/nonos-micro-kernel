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

//! Ribbon press handling: open or close a pill, run a toggle, or report a
//! paragraph icon the block model cannot express yet.

use nonos_app_skeleton::EventOutcome;

use super::hit::{ribbon_hit, RibbonHit};
use super::items::RibbonItem;
use crate::editor::app::Editor;

impl Editor {
    pub(in crate::editor) fn ribbon_press(&mut self, x: i32, y: i32) -> EventOutcome {
        match ribbon_hit(&self.rb_layout, self.rb_open, x, y) {
            RibbonHit::Cell(RibbonItem::Pill(p)) => {
                self.rb_open = if self.rb_open == Some(p) { None } else { Some(p) };
            }
            RibbonHit::Cell(RibbonItem::Toggle(t)) => {
                self.rb_open = None;
                self.apply_toggle(t);
            }
            RibbonHit::Cell(RibbonItem::Icon(_)) => {
                self.rb_open = None;
                self.doc().status = b"not implemented yet";
            }
            RibbonHit::Row(r) => {
                let pill = self.rb_open.take().unwrap_or(0);
                self.apply_pill_row(pill, r);
            }
            RibbonHit::Outside => self.rb_open = None,
        }
        EventOutcome::Repaint
    }
}
