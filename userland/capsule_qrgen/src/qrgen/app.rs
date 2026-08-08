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

use super::code::{encode, Matrix};
use super::event::on_event;
use super::manifest::manifest;
use super::paint::paint;

pub struct QrGen {
    matrix: Option<Matrix>,
}

impl QrGen {
    pub fn new() -> Self {
        QrGen { matrix: encode() }
    }
}

impl App for QrGen {
    fn manifest(&self) -> AppManifest {
        manifest()
    }

    fn on_event(&mut self, event: InputEvent) -> EventOutcome {
        on_event(event)
    }

    fn paint(&mut self, fb: &mut PaintBuffer) {
        paint(fb, self.matrix.as_ref());
    }
}
