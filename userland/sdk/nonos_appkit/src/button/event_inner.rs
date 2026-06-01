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

use nonos_abi::{InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use super::types::Button;

impl Button {
    pub(super) fn event_inner(&mut self, event: &InputEvent) -> bool {
        let x0 = self.rect.x as i32;
        let y0 = self.rect.y as i32;
        let inside = event.x >= x0
            && event.x < x0 + self.rect.w as i32
            && event.y >= y0
            && event.y < y0 + self.rect.h as i32;
        match event.kind {
            INPUT_KIND_BUTTON_DOWN if inside => {
                self.pressed = true;
                true
            }
            INPUT_KIND_BUTTON_UP => {
                let was = self.pressed;
                self.pressed = false;
                if was && inside {
                    self.clicked = true;
                }
                was
            }
            _ => false,
        }
    }
}
