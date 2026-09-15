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

use super::icon_path::Icon;
use super::paint_sidebar::PLACES;
use super::screen::Screen;
use super::sidebar_model::{SideHit, DOWNLOADS};

/// The glyph a navigation row wears. Downloads is a directory but reads as a
/// destination, so it keeps the arrow rather than the generic folder, and the
/// capsule store gets the cube it is drawn with everywhere else.
pub fn nav_icon(hit: &SideHit) -> Icon {
    match hit {
        SideHit::Screen(Screen::Home) => Icon::Home,
        SideHit::Screen(Screen::Recents) => Icon::Clock,
        SideHit::Screen(Screen::Shared) => Icon::People,
        SideHit::Screen(Screen::Tags) => Icon::Tag,
        SideHit::Screen(Screen::Search) => Icon::Magnifier,
        SideHit::Screen(_) => Icon::Folder,
        SideHit::Path(p) if p == DOWNLOADS => Icon::Download,
        SideHit::Path(p) if p == PLACES[1].1 => Icon::Doc,
        SideHit::Path(p) if p == PLACES[2].1 => Icon::Cube,
        SideHit::Path(_) => Icon::Folder,
    }
}
