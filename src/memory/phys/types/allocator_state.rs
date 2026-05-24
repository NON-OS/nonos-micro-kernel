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

#[derive(Debug)]
pub struct AllocatorState {
    pub frame_start: u64,
    pub frame_count: usize,
    pub bitmap_ptr: *mut u8,
    pub bitmap_bytes: usize,
    pub next_hint: u64,
    pub random_seed: u64,
}

impl AllocatorState {
    pub const fn new() -> Self {
        Self {
            frame_start: 0,
            frame_count: 0,
            bitmap_ptr: core::ptr::null_mut(),
            bitmap_bytes: 0,
            next_hint: 0,
            random_seed: 0,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.frame_count > 0 && !self.bitmap_ptr.is_null()
    }

    pub fn contig_split(&self) -> usize {
        let reserve = core::cmp::min(self.frame_count / 4, CONTIG_RESERVE_FRAMES);
        self.frame_count - reserve
    }
}

const CONTIG_RESERVE_FRAMES: usize = 32768;

impl Default for AllocatorState {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for AllocatorState {}
unsafe impl Sync for AllocatorState {}
