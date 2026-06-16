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

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use crate::tcp::{seq, FLAG_FIN};

pub struct RetxSeg {
    pub seq: u32,
    pub flags: u8,
    pub data: Vec<u8>,
    pub sent_ms: u64,
    pub xmits: u8,
}

pub struct RetxQueue {
    segs: VecDeque<RetxSeg>,
}

impl RetxQueue {
    pub const fn new() -> Self {
        RetxQueue { segs: VecDeque::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.segs.is_empty()
    }

    pub fn push(&mut self, seg: RetxSeg) {
        self.segs.push_back(seg);
    }

    pub fn oldest_mut(&mut self) -> Option<&mut RetxSeg> {
        self.segs.front_mut()
    }

    pub fn ack(&mut self, una: u32) -> usize {
        let mut acked = 0;
        while let Some(front) = self.segs.front() {
            let fin = if front.flags & FLAG_FIN != 0 { 1 } else { 0 };
            let end = front.seq.wrapping_add(front.data.len() as u32).wrapping_add(fin);
            if seq::leq(end, una) {
                self.segs.pop_front();
                acked += 1;
            } else {
                break;
            }
        }
        acked
    }
}
