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

const MAX_ATTR_PX: u32 = 4096;

// Whole-pixel value of an <img> width/height attribute. Percentages and
// malformed values yield None and the box falls back to its default size.
pub(super) fn attr_px(v: Option<&str>) -> Option<u32> {
    let n = v?.trim().parse::<u32>().ok()?;
    if n == 0 {
        return None;
    }
    Some(n.min(MAX_ATTR_PX))
}
