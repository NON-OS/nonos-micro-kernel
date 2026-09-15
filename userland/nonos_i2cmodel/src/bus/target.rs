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

//! What a device on the bus has to answer.

/// The direction bit that follows the address on the wire.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    Write,
    Read,
}

/// A device at one address. The master calls these in wire order: a start
/// (repeated or not) with the direction, then writes or reads, then a stop.
/// A device that is present acknowledges its address by being found; what it
/// acknowledges after that is its own business.
pub trait Target {
    fn address(&self) -> u8;
    /// A START or repeated START addressed to this device.
    fn start(&mut self, dir: Dir, repeated: bool);
    /// A data byte from the master. False is a NACK.
    fn write(&mut self, byte: u8) -> bool;
    /// A data byte for the master. `last` is true when the master will NACK
    /// it, which is how a device learns the read is over.
    fn read(&mut self, last: bool) -> u8;
    fn stop(&mut self);
}
