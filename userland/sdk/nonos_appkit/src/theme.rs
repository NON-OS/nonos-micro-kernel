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

use nonos_ui::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub button_bg: Color,
    pub button_fg: Color,
}

impl Theme {
    pub const fn dark() -> Theme {
        Theme {
            background: Color::rgb(0x1E, 0x1E, 0x2E),
            foreground: Color::rgb(0xCD, 0xD6, 0xF4),
            accent: Color::rgb(0x89, 0xB4, 0xFA),
            button_bg: Color::rgb(0x31, 0x32, 0x44),
            button_fg: Color::rgb(0xCD, 0xD6, 0xF4),
        }
    }
}
