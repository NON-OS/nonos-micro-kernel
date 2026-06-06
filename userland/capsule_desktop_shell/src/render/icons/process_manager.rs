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

pub fn process_manager(ctx: &Context, x: u32, y: u32, size: u32) {
    let fg = super::constants::ICON_FG;
    super::paint::paint_u(ctx, x, y, size, 3, 10, 2, 2, fg);
    super::paint::paint_u(ctx, x, y, size, 6, 7, 2, 5, fg);
    super::paint::paint_u(ctx, x, y, size, 9, 5, 2, 7, fg);
    super::paint::paint_u(ctx, x, y, size, 12, 3, 2, 9, fg);
}
