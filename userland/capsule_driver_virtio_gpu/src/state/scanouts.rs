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
use core::cell::Cell;
use crate::constants::VG_MAX_SCANOUTS;
#[derive(Clone, Copy, Default)]
pub struct Scanout {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub current_resource_id: u32,
    pub enabled: bool,
}
pub struct ScanoutTable {
    entries: [Cell<Scanout>; VG_MAX_SCANOUTS],
}
impl ScanoutTable {
    pub const fn new() -> Self {
        const EMPTY: Cell<Scanout> = Cell::new(Scanout {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            current_resource_id: 0,
            enabled: false,
        });
        Self { entries: [EMPTY; VG_MAX_SCANOUTS] }
    }
    pub fn record(&self, scanout_id: u32, s: Scanout) -> bool {
        let idx = scanout_id as usize;
        if idx >= VG_MAX_SCANOUTS {
            return false;
        }
        self.entries[idx].set(s);
        true
    }
    pub fn get(&self, scanout_id: u32) -> Option<Scanout> {
        let idx = scanout_id as usize;
        if idx >= VG_MAX_SCANOUTS {
            return None;
        }
        Some(self.entries[idx].get())
    }
}