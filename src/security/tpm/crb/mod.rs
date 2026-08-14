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

//! Command Response Buffer transport, at runtime.
//!
//! Every firmware TPM presents this interface: Intel PTT, AMD fTPM and QEMU's
//! `tpm-crb` all do. The part publishes a buffer in memory and a doorbell
//! rather than accepting bytes through a port.

mod buffer;
mod exec;
mod locality;
mod regs;
mod response;
mod transact;
mod wait;
mod window;

pub use transact::transact;
