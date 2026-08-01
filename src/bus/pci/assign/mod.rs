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

//! Programs the BARs of devices firmware left unassigned.
//!
//! A PC boots through UEFI, which assigns every BAR before the kernel runs, so
//! this does nothing there. A board booted straight from a device tree has no
//! such firmware: every BAR reads back zero and the device decodes nothing, so
//! a driver that trusts the BAR ends up pointing at address zero.

mod access;
mod carve;
mod device;
mod run;
mod size;
mod window;

pub use run::assign_unassigned;
pub use window::set_windows;
