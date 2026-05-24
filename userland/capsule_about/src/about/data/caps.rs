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

pub struct CapDescriptor {
    pub bit: u64,
    pub name: &'static [u8],
    pub role: &'static [u8],
}

pub const ALL_CAPS: &[CapDescriptor] = &[
    CapDescriptor { bit: 1 << 0,  name: b"CoreExec",             role: b"run user code" },
    CapDescriptor { bit: 1 << 1,  name: b"IO",                   role: b"raw port + console io" },
    CapDescriptor { bit: 1 << 2,  name: b"Network",              role: b"open sockets" },
    CapDescriptor { bit: 1 << 3,  name: b"IPC",                  role: b"toolkit calls + event recv" },
    CapDescriptor { bit: 1 << 4,  name: b"Memory",               role: b"mmap the paint buffer" },
    CapDescriptor { bit: 1 << 5,  name: b"Crypto",               role: b"call crypto primitives" },
    CapDescriptor { bit: 1 << 6,  name: b"FileSystem",           role: b"vfs read/write" },
    CapDescriptor { bit: 1 << 7,  name: b"Hardware",             role: b"raw hardware register access" },
    CapDescriptor { bit: 1 << 8,  name: b"Debug",                role: b"proof markers via MkDebug" },
    CapDescriptor { bit: 1 << 9,  name: b"Admin",                role: b"reboot/shutdown" },
    CapDescriptor { bit: 1 << 10, name: b"RegisterService",      role: b"publish an IPC endpoint" },
    CapDescriptor { bit: 1 << 11, name: b"GraphicsDisplayQuery", role: b"learn display dimensions" },
    CapDescriptor { bit: 1 << 12, name: b"GraphicsSurfaceCreate", role: b"register the paint surface" },
    CapDescriptor { bit: 1 << 13, name: b"GraphicsSurfaceMap",   role: b"map a registered surface" },
    CapDescriptor { bit: 1 << 14, name: b"GraphicsPresent",      role: b"flush a surface for scanout" },
    CapDescriptor { bit: 1 << 15, name: b"DeviceEnum",           role: b"enumerate broker devices" },
    CapDescriptor { bit: 1 << 16, name: b"Driver",               role: b"claim a broker device" },
    CapDescriptor { bit: 1 << 17, name: b"Mmio",                 role: b"map device mmio" },
    CapDescriptor { bit: 1 << 18, name: b"Irq",                  role: b"bind a device irq" },
    CapDescriptor { bit: 1 << 19, name: b"Dma",                  role: b"grant a dma window" },
    CapDescriptor { bit: 1 << 20, name: b"Pio",                  role: b"raw port io" },
];

pub const MASK: u64 = (1 << 0) | (1 << 3) | (1 << 4) | (1 << 8) | (1 << 11) | (1 << 12);

pub fn is_granted(bit: u64) -> bool {
    MASK & bit != 0
}
