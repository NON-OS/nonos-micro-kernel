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

// A decoded raster: ARGB8888, row-major, top-down.
pub struct Decoded {
    pub w: u32,
    pub h: u32,
    pub px: Vec<u32>,
}

enum Status {
    Pending,
    Ready(Decoded),
    Failed,
}

struct Entry {
    url: String,
    status: Status,
}

// Ceiling on resident decoded pixels. Past this the store fails new images
// closed rather than growing the capsule heap without bound. Kept well under
// the process heap so a page's DOM, layout and transient fetch buffers always
// have room; the store is also reset on navigation so a prior page's rasters
// never squeeze out the current one.
const BYTE_BUDGET: usize = 16 * 1024 * 1024;

pub struct Store {
    entries: Vec<Entry>,
    bytes: usize,
}

impl Store {
    pub fn new() -> Self {
        Store { entries: Vec::new(), bytes: 0 }
    }

    // Drop every cached raster and free its budget. Called on navigation so a
    // new page starts with the full image budget available to it.
    pub fn reset(&mut self) {
        self.entries.clear();
        self.bytes = 0;
    }

    // True once the url has an entry in any state, so callers do not re-queue it.
    pub fn contains(&self, url: &str) -> bool {
        self.entries.iter().any(|e| e.url == url)
    }

    // The decoded raster for `url`, if it finished decoding.
    pub fn ready(&self, url: &str) -> Option<&Decoded> {
        self.entries.iter().find(|e| e.url == url).and_then(|e| match &e.status {
            Status::Ready(d) => Some(d),
            _ => None,
        })
    }

    pub(super) fn mark_pending(&mut self, url: &str) {
        if !self.contains(url) {
            self.entries.push(Entry { url: String::from(url), status: Status::Pending });
        }
    }

    pub(super) fn set_failed(&mut self, url: &str) {
        self.set(url, Status::Failed);
    }

    pub(super) fn set_ready(&mut self, url: &str, d: Decoded) {
        let cost = d.px.len().saturating_mul(4);
        if self.bytes.saturating_add(cost) > BYTE_BUDGET {
            self.set(url, Status::Failed);
            return;
        }
        self.bytes += cost;
        self.set(url, Status::Ready(d));
    }

    fn set(&mut self, url: &str, status: Status) {
        match self.entries.iter_mut().find(|e| e.url == url) {
            Some(e) => e.status = status,
            None => self.entries.push(Entry { url: String::from(url), status }),
        }
    }
}
