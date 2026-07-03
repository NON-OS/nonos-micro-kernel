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

// Em size in px for a heading-scale bucket: 1 body, 2 h2/h3, 3 h1. Paint and
// wrap both size text through here so painted runs match their layout boxes.
pub fn px_for(scale: u32) -> f32 {
    match scale {
        3 => 30.0,
        2 => 22.0,
        _ => 15.0,
    }
}
