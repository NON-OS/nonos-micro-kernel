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

//! Addresses a station can transmit from without naming the machine.
//!
//! A factory address is burned into an EEPROM or an efuse and is unique to one
//! piece of silicon, so a system that transmits it announces the same
//! identifier to every network it ever joins. That survives a reinstall, it
//! survives changing IP, and it survives keeping nothing on disk: an amnesic
//! system that broadcasts its factory address is anonymous everywhere except
//! the one field every access point logs.
//!
//! One implementation shared by every driver, because a copy per driver is how
//! one of them keeps transmitting the factory address while the others do not,
//! and the leak is only ever as narrow as the leakiest link.

// no_std for the capsules that link this; the host test build needs the test
// harness, which needs std.
#![cfg_attr(not(test), no_std)]

mod local;

#[cfg(kani)]
mod proofs;
#[cfg(test)]
mod tests;

pub use local::{apply, from_random, is_factory_assigned, is_local_unicast, MAC_LEN};
