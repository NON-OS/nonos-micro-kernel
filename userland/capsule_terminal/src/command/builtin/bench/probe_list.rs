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

//! What each row on the table means.

pub struct Probe {
    pub name: &'static [u8],
    pub what: &'static [u8],
}

pub const PROBES: [Probe; 3] = [
    Probe {
        name: b"syscall",
        what: b"entry and exit, measured on the cheapest call the kernel has",
    },
    Probe {
        name: b"clock",
        what: b"a syscall that reads kernel state, against one that reads a register",
    },
    Probe { name: b"yield", what: b"a trip through the scheduler and back" },
];

/// The round trip is described here with the rest so one table explains every
/// row, even though it is measured separately because it needs a peer.
pub const IPC_NOTE: Probe = Probe {
    name: b"ipc",
    what: b"call and reply through another capsule, the cost every service pays",
};
