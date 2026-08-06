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

use nonos_app_skeleton::{App, AppManifest, EventOutcome, InputEvent, PaintBuffer};

use super::doc::Doc;
use super::event::on_event;
use super::layout::{wrap, Line};
use super::manifest::manifest;
use super::measure::measure;
use super::paint::paint;
use super::theme::MARGIN;

pub struct MdView {
    doc: Doc,
    lines: Vec<Line>,
    wrapped_width: u32,
}

impl MdView {
    pub fn new() -> Self {
        MdView {
            doc: Doc::new(),
            lines: Vec::new(),
            wrapped_width: 0,
        }
    }

    fn relayout(&mut self, width: u32) {
        let reloaded = self.doc.ensure();
        if self.doc.blocks.is_empty() || !(reloaded || self.wrapped_width != width) {
            return;
        }
        self.wrapped_width = width;
        let content = (width as i32 - 2 * MARGIN).max(80);
        self.lines = wrap(&self.doc.blocks, content, measure);
    }
}

impl App for MdView {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        self.relayout(fb.width);
        paint(fb, &self.lines, self.doc.error);
    }
}
