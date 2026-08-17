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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    Home,
    Library,
    Playlists,
    Files,
    Settings,
    Player,
    Details,
}

pub const NAV: [Route; 5] =
    [Route::Home, Route::Library, Route::Playlists, Route::Files, Route::Settings];

impl Route {
    pub fn label(self) -> &'static str {
        match self {
            Route::Home => "Home",
            Route::Library => "Library",
            Route::Playlists => "Playlists",
            Route::Files => "Files",
            Route::Settings => "Settings",
            Route::Player => "Now Playing",
            Route::Details => "Media Details",
        }
    }

    pub fn in_nav(self) -> bool {
        NAV.iter().any(|r| *r == self)
    }

    pub fn chrome(self) -> bool {
        !matches!(self, Route::Player)
    }
}
