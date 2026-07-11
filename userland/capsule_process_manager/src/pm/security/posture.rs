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

use super::super::state::Row;
use super::sensitive::{ADMIN, DEBUG, DMA, RAW_HW, SPAWN};

// A live count of how many processes hold each sensitive authority. This is the
// honest attack-surface summary: not a verdict, just the true tally over the
// whole running set, recomputed every refresh.
#[derive(Clone, Copy, Default)]
pub struct Posture {
    pub total: u32,
    pub admin: u32,
    pub raw_hw: u32,
    pub dma: u32,
    pub spawn: u32,
    pub debug: u32,
}

impl Posture {
    pub fn compute(rows: &[Row]) -> Self {
        let mut p = Posture { total: rows.len() as u32, ..Posture::default() };
        for r in rows {
            if r.caps & ADMIN != 0 {
                p.admin += 1;
            }
            if r.caps & RAW_HW != 0 {
                p.raw_hw += 1;
            }
            if r.caps & DMA != 0 {
                p.dma += 1;
            }
            if r.caps & SPAWN != 0 {
                p.spawn += 1;
            }
            if r.caps & DEBUG != 0 {
                p.debug += 1;
            }
        }
        p
    }
}
