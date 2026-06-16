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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::tcp::{seq, REASM_MAX_SEGS};

pub struct Reasm {
    segs: BTreeMap<u32, Vec<u8>>,
}

impl Reasm {
    pub const fn new() -> Self {
        Reasm { segs: BTreeMap::new() }
    }

    pub fn insert(&mut self, s: u32, data: Vec<u8>) {
        if data.is_empty() || self.segs.len() >= REASM_MAX_SEGS {
            return;
        }
        self.segs.entry(s).or_insert(data);
    }

    pub fn drain_contiguous(&mut self, mut rcv_nxt: u32) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let key = match self.segs.keys().next().copied() {
                Some(k) => k,
                None => break,
            };
            if seq::leq(key, rcv_nxt) {
                let data = self.segs.remove(&key).unwrap();
                let end = key.wrapping_add(data.len() as u32);
                if seq::gt(end, rcv_nxt) {
                    let skip = rcv_nxt.wrapping_sub(key) as usize;
                    out.extend_from_slice(&data[skip..]);
                    rcv_nxt = end;
                }
            } else {
                break;
            }
        }
        out
    }
}
