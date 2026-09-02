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

use crate::about::section::Section;

use super::nav_geom;

// The one routing surface. About has a single interactive region — the sidebar
// rail — so the pane is deliberately inert: every screen is evidence to read,
// not controls to press. Keeping the router here anyway means Phase B adds a
// target by extending one match, not by teaching the event layer geometry.
pub fn at(x: i32, y: i32) -> Option<Section> {
    nav_geom::at(x, y)
}
