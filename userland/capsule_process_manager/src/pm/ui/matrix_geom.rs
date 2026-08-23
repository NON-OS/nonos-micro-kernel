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

use crate::pm::security::sensitive as sens;

// The ten sensitive authorities, grouped as sensitive.rs groups them. The label
// is the column header, abbreviated because MIN_UI_PX clamps the face at 17 px
// and "spawnbroker" is wider than any column here; auth_legend spells them out.
pub const MATRIX: [(u64, &[u8]); 10] = [
    (sens::ADMIN, b"ADM"),
    (sens::DEBUG, b"DBG"),
    (sens::DRIVER, b"DRV"),
    (sens::MMIO, b"MMIO"),
    (sens::IRQ, b"IRQ"),
    (sens::DMA, b"DMA"),
    (sens::PIO, b"PIO"),
    (sens::SPAWN_BROKER, b"SPWB"),
    (sens::SPAWN_WINDOW, b"SPWW"),
    (sens::PROC_CTL, b"PCTL"),
];

pub const PAD_X: u32 = 14;
pub const NAME_W: u32 = 232;
pub const HEAD_H: u32 = 34;
pub const ROW_H: u32 = 26;
pub const CELL_DOT: u32 = 12;
pub const LEGEND_H: u32 = 48;

pub type Cell = (usize, usize);

pub fn cell_w(table_w: u32) -> u32 {
    table_w.saturating_sub(PAD_X * 2 + NAME_W) / MATRIX.len() as u32
}

pub fn cell_x(table_w: u32, col: usize) -> u32 {
    PAD_X + NAME_W + col as u32 * cell_w(table_w)
}

pub fn row_y(index: usize) -> u32 {
    HEAD_H + index as u32 * ROW_H
}

pub fn visible_rows(table_h: u32) -> usize {
    (table_h.saturating_sub(HEAD_H + LEGEND_H) / ROW_H) as usize
}

pub fn row_at(y: i32, scroll: usize, total: usize) -> Option<usize> {
    let offset = u32::try_from(y - HEAD_H as i32).ok()?;
    let index = scroll + (offset / ROW_H) as usize;
    (index < total).then_some(index)
}

// Table-local coordinates, matching table_geom. The caller bounds y by
// visible_rows first: the legend strip under the grid is not part of the matrix
// and this signature carries no height with which to reject it.
pub fn cell_at(table_w: u32, x: i32, y: i32, scroll: usize, total: usize) -> Option<Cell> {
    let row = row_at(y, scroll, total)?;
    let span = (x.max(0) as u32).checked_sub(PAD_X + NAME_W)?;
    let col = (span / cell_w(table_w).max(1)) as usize;
    (col < MATRIX.len()).then_some((row, col))
}
