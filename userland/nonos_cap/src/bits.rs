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

pub const CAP_CORE_EXEC: u64 = 1;
pub const CAP_IO: u64 = 2;
pub const CAP_NETWORK: u64 = 4;
pub const CAP_IPC: u64 = 8;
pub const CAP_MEMORY: u64 = 16;
pub const CAP_CRYPTO: u64 = 32;
pub const CAP_FILESYSTEM: u64 = 64;
pub const CAP_HARDWARE: u64 = 128;
pub const CAP_DEBUG: u64 = 256;
pub const CAP_ADMIN: u64 = 512;
pub const CAP_REGISTER_SERVICE: u64 = 1024;
pub const CAP_GRAPHICS_DISPLAY_QUERY: u64 = 2048;
pub const CAP_GRAPHICS_SURFACE_CREATE: u64 = 4096;
pub const CAP_GRAPHICS_SURFACE_MAP: u64 = 8192;
pub const CAP_GRAPHICS_PRESENT: u64 = 16384;
pub const CAP_DEVICE_ENUM: u64 = 32768;
pub const CAP_DRIVER: u64 = 65536;
pub const CAP_MMIO: u64 = 131072;
pub const CAP_IRQ: u64 = 262144;
pub const CAP_DMA: u64 = 524288;
pub const CAP_PIO: u64 = 1048576;
pub const CAP_INPUT_SOURCE: u64 = 2097152;
