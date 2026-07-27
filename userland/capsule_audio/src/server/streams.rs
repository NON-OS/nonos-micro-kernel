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

use super::ring::{PcmRing, FEED_SAMPLES};

pub const MAX_STREAMS: usize = 4;
pub const E_OK: i32 = 0;
pub const E_AGAIN: i32 = -11;
pub const E_INVAL: i32 = -22;

pub struct StreamSlot { pub id: u32, pub format: u16, pub paused: bool, pub feed: PcmRing }

pub struct StreamTable { slots: [Option<StreamSlot>; MAX_STREAMS], next_id: u32 }

impl StreamTable {
    pub fn new() -> Self {
        Self { slots: [None, None, None, None], next_id: 1 }
    }

    fn find(&mut self, id: u32) -> Option<&mut StreamSlot> {
        self.slots.iter_mut().flatten().find(|s| s.id == id)
    }

    pub fn open(&mut self, format: u16) -> Option<u32> {
        let free = self.slots.iter().position(|s| s.is_none())?;
        let id = self.next_id;
        self.next_id += 1;
        self.slots[free] = Some(StreamSlot { id, format, paused: false, feed: PcmRing::new() });
        Some(id)
    }

    pub fn close(&mut self, id: u32) -> bool {
        match self.slots.iter().position(|s| s.as_ref().map_or(false, |s| s.id == id)) {
            Some(i) => { self.slots[i] = None; true }
            None => false,
        }
    }

    pub fn set_paused(&mut self, id: u32, p: bool) -> bool {
        match self.find(id) {
            Some(s) => {
                s.paused = p;
                if p {
                    s.feed.clear();
                }
                true
            }
            None => false,
        }
    }

    pub fn any_active(&self) -> bool {
        self.slots.iter().flatten().any(|s| !s.paused)
    }

    pub fn feed(&mut self, id: u32, s: &[i16]) -> i32 {
        match self.find(id) {
            Some(st) if FEED_SAMPLES - st.feed.len() >= s.len() => { st.feed.push(s); E_OK }
            Some(_) => E_AGAIN,
            None => E_INVAL,
        }
    }

    pub fn iter_active(&mut self) -> impl Iterator<Item = &mut StreamSlot> {
        self.slots.iter_mut().flatten().filter(|s| !s.paused)
    }
}
