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

pub fn terminal(ctx: &Context, x: u32, y: u32) {
    let fg = super::constants::ICON_FG;
    super::paint::paint(ctx, x + 2, y + 3, 12, 1, fg);
    super::paint::paint(ctx, x + 2, y + 4, 1, 8, fg);
    super::paint::paint(ctx, x + 13, y + 4, 1, 8, fg);
    super::paint::paint(ctx, x + 3, y + 11, 10, 1, fg);
    super::paint::paint(ctx, x + 5, y + 6, 2, 2, fg);
    super::paint::paint(ctx, x + 7, y + 8, 2, 1, fg);
    super::paint::paint(ctx, x + 9, y + 9, 2, 1, fg);
    super::paint::paint(ctx, x + 8, y + 10, 3, 1, fg);
}
