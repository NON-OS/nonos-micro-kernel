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
use super::screen::Screen;
use super::theme::{AMBER, BLUE_LT, CY, GRN, INK3};

/// One Home category. `path` is both the store prefix its figure is counted
/// from and the place a click browses to; `screen` is a surface a click switches
/// to instead. `counted` says whether anything behind the card can produce a
/// figure at all -- a card with no backend draws dimmed and states that, rather
/// than showing a number nothing measured.
pub struct Cat {
    pub title: &'static str,
    pub icon: Icon,
    pub tint: u32,
    pub path: Option<&'static str>,
    pub screen: Option<Screen>,
    pub counted: bool,
}

const fn cat(
    title: &'static str,
    icon: Icon,
    tint: u32,
    path: Option<&'static str>,
    screen: Option<Screen>,
    counted: bool,
) -> Cat {
    Cat { title, icon, tint, path, screen, counted }
}

/// The category row, in drawn order. Recents counts the access journal, the
/// three prefix cards count their own subtree, and Shared has no identity
/// backend to count against so it carries no figure.
pub const CATS: [Cat; 5] = [
    cat("Recents", Icon::Clock, BLUE_LT, None, Some(Screen::Recents), true),
    cat("Projects", Icon::Folder, AMBER, Some("/projects/"), None, true),
    cat("Downloads", Icon::Download, GRN, Some("/downloads/"), None, true),
    cat("Shared", Icon::People, INK3, None, Some(Screen::Shared), false),
    cat("Capsules", Icon::Cube, CY, Some("/capsules/"), None, true),
];
