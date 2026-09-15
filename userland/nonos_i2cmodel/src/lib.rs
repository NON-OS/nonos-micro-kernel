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

//! An I2C controller, a bus and a device, for a driver to talk to on the host.
//!
//! A register window in memory is enough to prove a bring-up sequence, and
//! `nonos_devmodel` does that. It is not enough for a transfer. The DesignWare
//! core's data register is a FIFO on both sides: a write pushes a command, a
//! read pops a byte, and reading the abort-clear register clears the abort.
//! Memory cannot see a read, so a model behind memory cannot know when a byte
//! was taken or an abort acknowledged.
//!
//! This crate is the other arrangement. [`Designware`] is the core with every
//! register's read and write semantics written from the databook, and it is
//! reached through the driver's own register accessor, which the proof crates
//! replace with one that calls in here. Below the core sits a [`Bus`] with
//! [`Target`]s on it, and [`HidOverI2c`] is a touchpad that speaks the
//! HID-over-I2C protocol on that bus. The controller executes its command
//! queue while the driver looks at a register, the way a real part works
//! while the software polls, so a burst of commands pushed without checking
//! the FIFO depth overflows here as it would there.
//!
//! What every layer records is what the proofs assert on: the order of
//! register writes, the bytes on the bus with their acknowledgements, the
//! commands the device received, and every act the specification forbids.

mod bus;
mod designware;
mod hid;

pub use bus::{Bus, BusEvent, Dir, Shared, Target};
pub use designware::{regs, Config, Designware, RegEvent, Violation};
pub use hid::{descriptor, touchpad, Command, HidOverI2c};

#[cfg(test)]
mod tests;
