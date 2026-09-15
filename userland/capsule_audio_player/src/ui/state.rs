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

//! View state for the shell: which screen is up, where each list is scrolled,
//! and the live search buffer fed from the keyboard path.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use crate::library::Library;

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Home,
    Library,
    Browse,
    Search,
    Radio,
    NowPlaying,
    Downloads,
    Settings,
}

pub const NAV: [(View, &str); 7] = [
    (View::Home, "Home"),
    (View::Library, "Library"),
    (View::Browse, "Browse"),
    (View::Search, "Search"),
    (View::Radio, "Radio"),
    (View::NowPlaying, "Now Playing"),
    (View::Downloads, "Downloads"),
];

pub const PLAYLISTS: [(&str, u32); 5] = [
    ("chillwave", 72),
    ("midnight drive", 54),
    ("focus mode", 88),
    ("synthwave", 62),
    ("late night", 36),
];

pub const RAIL_TABS: [&str; 3] = ["Up next", "Lyrics", "Related"];

pub const SET_SECTIONS: [&str; 9] = [
    "Playback",
    "Audio quality",
    "Downloads",
    "Appearance",
    "Notifications",
    "Devices",
    "Account",
    "Privacy",
    "About",
];

pub const LIB_TABS: [&str; 4] = ["Songs", "Formats", "Queued", "On disk"];

pub struct UiState {
    pub view: View,
    pub scroll: usize,
    pub query: String,
    pub lib_tab: usize,
    pub rail_tab: usize,
    pub set_sec: usize,
    pub playlist: Option<usize>,
    pub hover: Option<usize>,
}

impl UiState {
    pub fn new() -> UiState {
        UiState {
            view: View::Home,
            scroll: 0,
            query: String::new(),
            lib_tab: 0,
            rail_tab: 0,
            set_sec: 0,
            playlist: None,
            hover: None,
        }
    }

    pub fn go(&mut self, v: View) {
        self.view = v;
        self.scroll = 0;
    }

    pub fn scroll_by(&mut self, delta: i32, len: usize, visible: usize) {
        let max = len.saturating_sub(visible);
        let next = self.scroll as i32 + delta;
        self.scroll = next.clamp(0, max as i32) as usize;
    }
}

fn folded(hay: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let hb = hay.as_bytes();
    let nb = needle.as_bytes();
    if nb.len() > hb.len() {
        return false;
    }
    (0..=hb.len() - nb.len()).any(|i| {
        (0..nb.len()).all(|j| hb[i + j].eq_ignore_ascii_case(&nb[j]))
    })
}

pub fn matches(lib: &Library, i: usize, q: &str) -> bool {
    match lib.get(i) {
        Some(t) => folded(&t.title, q) || folded(&t.artist, q) || folded(&t.format, q),
        None => false,
    }
}

pub fn filtered(lib: &Library, q: &str) -> Vec<usize> {
    (0..lib.tracks.len()).filter(|&i| matches(lib, i, q)).collect()
}
