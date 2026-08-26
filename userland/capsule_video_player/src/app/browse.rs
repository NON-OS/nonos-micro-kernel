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

use alloc::string::String;
use alloc::vec::Vec;

use crate::catalog::media::MediaItem;

pub struct Browse {
    pub items: Vec<MediaItem>,
    pub view: Vec<usize>,
    pub sel: usize,
    pub scroll: usize,
    pub scanned: bool,
    pub query: String,
    pub grid: bool,
}

impl Browse {
    pub fn new() -> Browse {
        Browse {
            items: Vec::new(),
            view: Vec::new(),
            sel: 0,
            scroll: 0,
            scanned: false,
            query: String::new(),
            grid: true,
        }
    }

    pub fn len(&self) -> usize {
        self.view.len()
    }

    pub fn is_empty(&self) -> bool {
        self.view.is_empty()
    }

    pub fn get(&self, slot: usize) -> Option<&MediaItem> {
        self.items.get(*self.view.get(slot)?)
    }

    pub fn selected(&self) -> Option<&MediaItem> {
        self.get(self.sel)
    }

    pub fn reindex(&mut self) {
        self.view.clear();
        for (i, item) in self.items.iter().enumerate() {
            if matches(item, &self.query) {
                self.view.push(i);
            }
        }
        if self.sel >= self.view.len() {
            self.sel = 0;
            self.scroll = 0;
        }
    }
}

fn matches(item: &MediaItem, query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let name = item.name.as_bytes();
    let needle = query.as_bytes();
    if needle.len() > name.len() {
        return false;
    }
    name.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle))
}
