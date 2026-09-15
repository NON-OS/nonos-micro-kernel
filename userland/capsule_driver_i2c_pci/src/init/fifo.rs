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

//! The FIFO depths, read from the core rather than assumed. Intel's LPSS
//! instances carry 64 entries each way, but the DesignWare core is built with
//! 8, 16 and 32 as well, and a transfer engine that pushes to an assumed
//! depth loses commands on every part shallower than it guessed.

use crate::constants::{COMP_PARAM_RX_DEPTH_SHIFT, COMP_PARAM_TX_DEPTH_SHIFT};

/// (TX depth, RX depth) in entries, from IC_COMP_PARAM_1, which stores each
/// depth minus one in an eight-bit field.
pub fn fifo_depths(comp_param: u32) -> (u32, u32) {
    let tx = ((comp_param >> COMP_PARAM_TX_DEPTH_SHIFT) & 0xFF) + 1;
    let rx = ((comp_param >> COMP_PARAM_RX_DEPTH_SHIFT) & 0xFF) + 1;
    (tx, rx)
}
