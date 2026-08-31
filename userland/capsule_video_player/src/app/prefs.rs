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

pub const SECTIONS: [&str; 9] = [
    "General",
    "Playback",
    "Subtitles",
    "Audio",
    "Library",
    "Appearance",
    "Performance",
    "Shortcuts",
    "Privacy",
];

pub const TOGGLES: [&str; 12] = [
    "Remember Last Position",
    "Show Time in Titlebar",
    "Hardware Acceleration",
    "Allow Background Playback",
    "Preload Next Video",
    "Auto-Load Subtitles",
    "Enable Subtitles by Default",
    "Normalize Volume",
    "Audio Boost",
    "Automatically Scan",
    "Generate Thumbnails",
    "Show Mini Player",
];

pub const SECTION_SPAN: [(usize, usize); SECTIONS.len()] =
    [(0, 2), (2, 3), (5, 2), (7, 2), (9, 2), (11, 1), (12, 0), (12, 0), (12, 0)];

const DEFAULTS: u16 = 0b0000_0110_1011_0101;

pub struct Prefs {
    pub section: usize,
    pub flags: u16,
}

impl Prefs {
    pub fn new() -> Prefs {
        Prefs { section: 0, flags: DEFAULTS }
    }

    pub fn get(&self, index: usize) -> bool {
        index < TOGGLES.len() && self.flags & (1 << index) != 0
    }

    pub fn toggle(&mut self, index: usize) -> bool {
        if index >= TOGGLES.len() {
            return false;
        }
        self.flags ^= 1 << index;
        true
    }

    pub fn reset(&mut self) {
        self.flags = DEFAULTS;
        self.section = 0;
    }
}
