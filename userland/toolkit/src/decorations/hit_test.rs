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

use super::close_button::{close_button_rect, maximize_button_rect, minimize_button_rect};
use super::metrics::TITLEBAR_HEIGHT;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DecorationHit {
    None,
    Titlebar,
    CloseButton,
    MinimizeButton,
    MaximizeButton,
}

pub fn hit_test(width: u32, x: u32, y: u32) -> DecorationHit {
    if y >= TITLEBAR_HEIGHT {
        return DecorationHit::None;
    }
    let c = close_button_rect(width);
    if x >= c.x && x < c.x + c.size && y >= c.y && y < c.y + c.size {
        return DecorationHit::CloseButton;
    }
    let m = maximize_button_rect(width);
    if x >= m.x && x < m.x + m.size && y >= m.y && y < m.y + m.size {
        return DecorationHit::MaximizeButton;
    }
    let n = minimize_button_rect(width);
    if x >= n.x && x < n.x + n.size && y >= n.y && y < n.y + n.size {
        return DecorationHit::MinimizeButton;
    }
    DecorationHit::Titlebar
}
