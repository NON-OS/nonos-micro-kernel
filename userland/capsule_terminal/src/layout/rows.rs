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

pub fn row_top(i: u32, body_y: u32, lh: u32) -> u32 {
    body_y + i * lh
}

pub fn scroll_max(content_h: u32, viewport_h: u32) -> u32 {
    content_h.saturating_sub(viewport_h)
}

pub fn scroll_clamp(offset: u32, content_h: u32, viewport_h: u32) -> u32 {
    offset.min(scroll_max(content_h, viewport_h))
}
