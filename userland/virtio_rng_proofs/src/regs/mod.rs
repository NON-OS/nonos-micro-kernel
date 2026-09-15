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

//! The driver's register accessors, assembled for the host.
//!
//! `state.rs` and `io.rs` are the shipping files. `pio` is the one part that
//! cannot come along: it issues port-IO syscalls through `nonos_libc`, which
//! has no host build. The shim below stands in for it and refuses to be
//! called, which is the honest arrangement rather than a silent stub. These
//! tests build a window with `Regs::mmio`, so the PIO arm is unreachable, and
//! if a future test takes it the panic says so instead of returning a zero
//! that would look like a device answering.

#[path = "../../../capsule_driver_virtio_rng/src/regs/io.rs"]
mod io;

mod pio;

#[path = "../../../capsule_driver_virtio_rng/src/regs/state.rs"]
mod state;

pub use self::state::Regs;
