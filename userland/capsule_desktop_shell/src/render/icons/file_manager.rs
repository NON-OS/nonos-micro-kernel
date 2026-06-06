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

use crate::state::Context;

pub fn file_manager(ctx: &Context, x: u32, y: u32) {
    let fg = super::constants::ICON_FG;
    let bg = super::constants::ICON_BG;
    super::paint::paint(ctx, x + 2, y + 5, 12, 7, fg);
    super::paint::paint(ctx, x + 4, y + 3, 4, 2, fg);
    super::paint::paint(ctx, x + 3, y + 6, 10, 1, bg);
    super::paint::paint(ctx, x + 3, y + 8, 8, 1, bg);
}
