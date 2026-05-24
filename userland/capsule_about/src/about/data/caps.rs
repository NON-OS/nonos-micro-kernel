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

pub const MASK: u64 = 0x1919;

pub const GRANTED: &[(&[u8], &[u8])] = &[
    (b"CoreExec", b"run user code"),
    (b"IPC", b"toolkit calls + event recv"),
    (b"Memory", b"mmap the paint buffer"),
    (b"Debug", b"proof markers via MkDebug"),
    (b"GraphicsDisplayQuery", b"learn display dimensions"),
    (b"GraphicsSurfaceCreate", b"register the paint surface"),
];

pub const DENIED: &[&[u8]] = &[
    b"IO", b"Network", b"Crypto", b"FileSystem", b"Hardware", b"Admin",
    b"RegisterService", b"Driver", b"Mmio", b"Irq", b"Dma", b"Pio",
    b"GraphicsSurfaceMap", b"GraphicsPresent", b"DeviceEnum",
];
