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

// The six screens the sidebar navigates. Overview and Processes are the two
// table screens; the rest are projections of the same snapshot.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Overview,
    Processes,
    Cpu,
    Memory,
    Authority,
    Security,
}

pub const SCREENS: [Screen; 6] = [
    Screen::Overview,
    Screen::Processes,
    Screen::Cpu,
    Screen::Memory,
    Screen::Authority,
    Screen::Security,
];

impl Screen {
    pub fn nav_label(self) -> &'static [u8] {
        match self {
            Screen::Overview => b"Overview",
            Screen::Processes => b"Processes",
            Screen::Cpu => b"CPU",
            Screen::Memory => b"Memory",
            Screen::Authority => b"Authority",
            Screen::Security => b"Security",
        }
    }

    // The Overview and Processes screens dock the per-process inspector; the
    // four projection screens hand that width back to their own content.
    pub fn has_inspector(self) -> bool {
        matches!(self, Screen::Overview | Screen::Processes)
    }
}
