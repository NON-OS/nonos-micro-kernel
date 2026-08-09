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

// Bit i holds the name of the capability whose kernel bit is `1 << i`. A
// hand-synced mirror of `src/capabilities/types/{defs,bit,as_str}.rs`: the
// order is `bit.rs`'s and the strings are `as_str.rs`'s verbatim, so a new
// capability must be appended here in the same position it takes there.
pub(super) const CAP_NAMES: [&[u8]; 27] = [
    b"CoreExec",
    b"IO",
    b"Network",
    b"IPC",
    b"Memory",
    b"Crypto",
    b"FileSystem",
    b"Hardware",
    b"Debug",
    b"Admin",
    b"RegisterService",
    b"GraphicsDisplayQuery",
    b"GraphicsSurfaceCreate",
    b"GraphicsSurfaceMap",
    b"GraphicsPresent",
    b"DeviceEnum",
    b"Driver",
    b"Mmio",
    b"Irq",
    b"Dma",
    b"Pio",
    b"InputSource",
    b"TimeSet",
    b"SpawnBroker",
    b"SpawnWindow",
    b"ProcessControl",
    b"StoreWrite",
];
