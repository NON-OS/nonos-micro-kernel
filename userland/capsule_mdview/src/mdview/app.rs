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

use super::event::on_event;
use super::layout::{parse, wrap, Block, Line};
use super::load::read_doc;
use super::manifest::manifest;
use super::measure::measure;
use super::paint::paint;
use super::theme::MARGIN;

pub struct MdView {
    blocks: Vec<Block>,
    lines: Vec<Line>,
    error: Option<&'static str>,
    wrapped_width: u32,
    loaded: bool,
}

impl MdView {
    pub fn new() -> Self {
        MdView {
            blocks: Vec::new(),
            lines: Vec::new(),
            error: None,
            wrapped_width: 0,
            loaded: false,
        }
    }

    fn relayout(&mut self, width: u32) {
        if !self.loaded {
            self.loaded = true;
            match read_doc() {
                Ok(text) => self.blocks = parse(&text),
                Err(message) => self.error = Some(message),
            }
        }
        if self.error.is_none() && self.wrapped_width != width {
            self.wrapped_width = width;
            let content = (width as i32 - 2 * MARGIN).max(80);
            self.lines = wrap(&self.blocks, content, measure);
        }
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
        paint(fb, &self.lines, self.error);
    }
}
