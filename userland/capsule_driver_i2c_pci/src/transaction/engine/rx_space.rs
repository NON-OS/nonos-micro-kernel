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
use crate::constants::IC_RXFLR;
use crate::regs::Regs;

/// Read commands the receive FIFO can still absorb. IC_RXFLR counts bytes
/// that have landed; `in_flight` is the reads already issued whose bytes have
/// not, and they need room too, or the core drops what arrives for them.
pub fn rx_space(regs: Regs, depth: u32, in_flight: u32) -> u32 {
    depth.saturating_sub(regs.read32(IC_RXFLR)).saturating_sub(in_flight)
}
