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

//! How this instance of the core was built. The FIFO depths are the part of
//! it a driver has to read rather than assume: the core reports them in
//! IC_COMP_PARAM_1 and silicon ships with 8, 16, 32 and 64.

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub tx_depth: u32,
    pub rx_depth: u32,
}

impl Config {
    /// Intel's LPSS instances carry 64-entry FIFOs both ways.
    pub const LPSS: Config = Config { tx_depth: 64, rx_depth: 64 };

    /// A core with the smallest FIFOs the databook allows. A driver that
    /// assumed the LPSS depths overruns this one.
    pub const SHALLOW: Config = Config { tx_depth: 8, rx_depth: 8 };

    /// IC_COMP_PARAM_1: TX depth minus one in bits 23:16, RX depth minus one
    /// in 15:8, fast mode as the maximum speed in 3:2, a 32-bit APB in 1:0.
    pub fn comp_param_1(&self) -> u32 {
        ((self.tx_depth - 1) << 16) | ((self.rx_depth - 1) << 8) | (2 << 2) | 0x2
    }
}
