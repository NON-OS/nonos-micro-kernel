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

use nonos_abi::InputEvent;
use nonos_ui::{Canvas, Control, Widget};

use super::types::Panel;

impl Control for Panel {
    fn paint(&self, canvas: &mut Canvas<'_>) {
        for label in &self.labels {
            label.paint(canvas);
        }
        for button in &self.buttons {
            button.paint(canvas);
        }
    }

    fn on_event(&mut self, event: &InputEvent) -> bool {
        let mut repaint = false;
        for button in &mut self.buttons {
            if button.on_event(event) {
                repaint = true;
            }
        }
        repaint
    }
}
