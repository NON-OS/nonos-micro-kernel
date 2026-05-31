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

//! Register accessors over the virtio legacy register window. The
//! window is reached either through an MMIO mapping (`user_va`) or
//! a PIO grant, depending on the BAR the broker handed the caller;
//! `Regs` hides the transport behind a uniform offset interface.

mod io;
mod pio;
mod state;

pub use self::state::Regs;
