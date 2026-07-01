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

use super::color::parse_color;
use super::computed::Computed;

pub fn apply_decl(c: &mut Computed, name: &str, value: &str) {
    match name {
        "color" => {
            if let Some(rgb) = parse_color(value) {
                c.color = rgb;
            }
        }
        "background" | "background-color" => {
            if let Some(rgb) = parse_color(value) {
                c.bg = rgb;
            }
        }
        "font-weight" => match value.trim() {
            "normal" | "100" | "200" | "300" | "400" => c.bold = false,
            "bold" | "bolder" | "500" | "600" | "700" | "800" | "900" => c.bold = true,
            _ => {}
        },
        "display" => {
            if value.trim() == "none" {
                c.display_none = true;
            }
        }
        _ => {}
    }
}
