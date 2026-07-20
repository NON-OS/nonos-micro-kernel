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

//! The best-effort data TX path for the RTL8821CE: the packet descriptor, the
//! buffer-descriptor ring bookkeeping, the register setup that points the card
//! at the ring, and the enqueue-and-kick that queues one frame. The 802.11
//! framing and rate/security choices that feed it arrive with the MLME wiring.

pub mod desc;
pub mod regs;
pub mod ring;
mod setup;
mod xmit;

pub use setup::program;
pub use xmit::enqueue;
