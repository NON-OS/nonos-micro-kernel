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
use super::items::{RibbonItem, ICON_ALIGN, ICON_TABLE};
use crate::editor::app::Editor;
use crate::editor::table_ops::{DEFAULT_COLS, DEFAULT_ROWS};
use crate::editor::unsupported::{NO_BLOCK_MODEL, NO_DOC_MODE};

impl Editor {
    pub(in crate::editor) fn ribbon_press(&mut self, x: i32, y: i32) -> EventOutcome {
        match ribbon_hit(&self.rb_layout, self.rb_open, x, y) {
            RibbonHit::Cell(RibbonItem::Pill(p)) => {
                self.rb_open = if self.rb_open == Some(p) { None } else { Some(p) };
            }
            RibbonHit::Cell(RibbonItem::Toggle(t)) => {
                self.rb_open = None;
                self.doc().apply_toggle(t);
            }
            RibbonHit::Cell(RibbonItem::Icon(k)) => {
                self.rb_open = None;
                match ICON_ALIGN.get(k).copied().flatten() {
                    Some(a) => self.doc().align_sel(a),
                    None if k == ICON_TABLE => self.insert_default_table(),
                    None => self.doc().status = NO_BLOCK_MODEL,
                }
            }
            RibbonHit::Row(r) => {
                let pill = self.rb_open.take().unwrap_or(0);
                self.doc().apply_pill_row(pill, r);
            }
            RibbonHit::Outside => self.rb_open = None,
        }
        EventOutcome::Repaint
    }

    fn insert_default_table(&mut self) {
        if !self.doc().insert_table(DEFAULT_ROWS, DEFAULT_COLS) {
            self.doc().status = NO_DOC_MODE;
        }
    }
}
