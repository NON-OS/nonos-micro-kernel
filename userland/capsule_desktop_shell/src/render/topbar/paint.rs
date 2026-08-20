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

//! Draw the whole menu bar in one call: background, brand, the menu titles,
//! then the live status cluster. Called on every chrome repaint, so the clock and battery
//! stay current.

use crate::state::Context;

pub fn paint(ctx: &Context) {
    super::background::background(ctx);
    super::brand::brand(ctx);
    crate::render::menubar_menu::paint_titles(ctx);
    super::status::status(ctx);
}
