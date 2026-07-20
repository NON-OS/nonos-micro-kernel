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

//! Host-to-card command paths to the firmware on the on-chip 8051. Short
//! commands go through the four hardware mailboxes (`mailbox`); larger offload
//! commands are 32-byte packets sent down the H2C transfer queue (`packet`), of
//! which the IQK calibration request is one.

pub mod mailbox;
pub mod packet;

pub use mailbox::H2c;
pub use packet::build_iqk;
