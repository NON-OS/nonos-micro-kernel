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

//! The MPDU RX path for the RTL8821CE: the RX descriptor parser, the ring
//! bookkeeping, the register setup that arms the ring, and the single-frame poll
//! that lifts received 802.11 frames off it. An interrupt-driven service loop
//! wraps the poll once the frames have a consumer (the MLME and the net_core
//! link contract).

pub mod desc;
mod poll;
pub mod regs;
pub mod ring;
mod setup;

pub use poll::poll_one;
pub use setup::program;
