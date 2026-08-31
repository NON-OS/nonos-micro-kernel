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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    Setup,
    Play,
    Pause,
    Over,
    Rank,
}

pub const ALL: [Screen; 6] =
    [Screen::Home, Screen::Setup, Screen::Play, Screen::Pause, Screen::Over, Screen::Rank];

impl Screen {
    pub fn title(self) -> &'static [u8] {
        match self {
            Screen::Home => b"Snake",
            Screen::Setup => b"New Run",
            Screen::Play => b"Playing",
            Screen::Pause => b"Paused",
            Screen::Over => b"Run Over",
            Screen::Rank => b"Ranks",
        }
    }

    // Pause and Over sit over a live board, so the board keeps painting under
    // them rather than being torn down.
    pub fn over_board(self) -> bool {
        matches!(self, Screen::Play | Screen::Pause | Screen::Over)
    }
}
