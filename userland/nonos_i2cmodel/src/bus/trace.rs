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

//! What crossed the wire, in order. The proofs read a transfer off this the
//! way an analyser would read it off SDA.

use super::target::Dir;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BusEvent {
    /// A START with an address and direction. `acked` is false when nothing
    /// at that address answered.
    Start {
        addr: u8,
        dir: Dir,
        acked: bool,
    },
    /// A repeated START: the bus was held, no STOP came between.
    Restart {
        addr: u8,
        dir: Dir,
        acked: bool,
    },
    Write {
        byte: u8,
        acked: bool,
    },
    Read {
        byte: u8,
        last: bool,
    },
    Stop,
}

impl BusEvent {
    pub fn is_start(&self) -> bool {
        matches!(self, Self::Start { .. })
    }
    pub fn is_restart(&self) -> bool {
        matches!(self, Self::Restart { .. })
    }
    pub fn is_stop(&self) -> bool {
        matches!(self, Self::Stop)
    }
    /// The data byte a write carried, if this was one.
    pub fn written(&self) -> Option<u8> {
        match self {
            Self::Write { byte, .. } => Some(*byte),
            _ => None,
        }
    }
    pub fn read_back(&self) -> Option<u8> {
        match self {
            Self::Read { byte, .. } => Some(*byte),
            _ => None,
        }
    }
}
