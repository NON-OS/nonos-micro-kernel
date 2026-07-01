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

pub fn named(name: &str) -> Option<u32> {
    let c: u32 = match name.to_ascii_lowercase().as_str() {
        "black" => 0x000000,
        "white" => 0xFFFFFF,
        "red" => 0xFF0000,
        "green" => 0x008000,
        "lime" => 0x00FF00,
        "blue" => 0x0000FF,
        "navy" => 0x000080,
        "gray" | "grey" => 0x808080,
        "silver" => 0xC0C0C0,
        "maroon" => 0x800000,
        "yellow" => 0xFFFF00,
        "orange" => 0xFFA500,
        "purple" => 0x800080,
        "teal" => 0x008080,
        "aqua" | "cyan" => 0x00FFFF,
        "fuchsia" | "magenta" => 0xFF00FF,
        "olive" => 0x808000,
        "darkgray" | "darkgrey" => 0xA9A9A9,
        "lightgray" | "lightgrey" => 0xD3D3D3,
        "transparent" => return Some(0),
        _ => return None,
    };
    Some(0xFF00_0000 | c)
}
