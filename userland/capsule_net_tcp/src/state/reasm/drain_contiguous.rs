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

use alloc::vec::Vec;

use crate::tcp::seq;

use super::Reasm;

impl Reasm {
    pub fn drain_contiguous(&mut self, mut rcv_nxt: u32) -> Vec<u8> {
        let mut out = Vec::new();
        loop {
            let Some(key) = self.segs.keys().next().copied() else { break; };
            if !seq::leq(key, rcv_nxt) {
                break;
            }
            let Some(data) = self.segs.remove(&key) else { break; };
            let end = key.wrapping_add(data.len() as u32);
            if seq::gt(end, rcv_nxt) {
                let skip = rcv_nxt.wrapping_sub(key) as usize;
                out.extend_from_slice(&data[skip..]);
                rcv_nxt = end;
            }
        }
        out
    }
}
