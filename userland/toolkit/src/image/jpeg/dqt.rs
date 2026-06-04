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

use crate::image::types::DecodeError;

pub const MAX_QT: usize = 4;

#[derive(Clone, Copy)]
pub struct QuantTable {
    pub present: bool,
    pub values: [u16; 64],
}

impl QuantTable {
    pub const fn new() -> Self {
        Self { present: false, values: [0; 64] }
    }
}

pub fn parse_dqt(seg: &[u8], tables: &mut [QuantTable; MAX_QT]) -> Result<(), DecodeError> {
    let mut p = 0usize;
    while p < seg.len() {
        let pq_tq = seg[p];
        p += 1;
        let pq = (pq_tq >> 4) & 0x0F;
        let tq = (pq_tq & 0x0F) as usize;
        if tq >= MAX_QT {
            return Err(DecodeError::Unsupported);
        }
        let entry_bytes = if pq == 0 {
            64
        } else if pq == 1 {
            128
        } else {
            return Err(DecodeError::Unsupported);
        };
        if p + entry_bytes > seg.len() {
            return Err(DecodeError::Truncated);
        }
        let mut t = QuantTable::new();
        if pq == 0 {
            let mut i = 0usize;
            while i < 64 {
                t.values[i] = seg[p + i] as u16;
                i += 1;
            }
        } else {
            let mut i = 0usize;
            while i < 64 {
                let hi = seg[p + i * 2] as u16;
                let lo = seg[p + i * 2 + 1] as u16;
                t.values[i] = (hi << 8) | lo;
                i += 1;
            }
        }
        t.present = true;
        tables[tq] = t;
        p += entry_bytes;
    }
    Ok(())
}
