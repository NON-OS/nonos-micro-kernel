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

pub const SLOT_CAP: usize = 256;
pub const INPUT_RING_CAP: usize = 1024;
pub const MAX_PAGES_PER_SURFACE: usize = 16384;
pub const PIXEL_BYTES: u32 = 4;

pub const FMT_ARGB8888: u32 = 1;

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SurfaceFormat {
    Argb8888 = FMT_ARGB8888,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SurfaceDescriptor {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
    pub byte_len: u64,
    pub base_va: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct InputEvent {
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub timestamp_ns: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RegistryError {
    InvalidArg,
    OutOfSlots,
    NotOwner,
    NotFound,
    BadHandle,
    MapFailed,
    NoProc,
}

pub type SurfaceSid = u64;
pub type SurfaceHandle = u64;

#[inline]
pub fn encode_handle(slot_idx: u32, epoch: u32) -> SurfaceHandle {
    ((slot_idx as u64) << 32) | epoch as u64
}

#[inline]
pub fn decode_handle(h: SurfaceHandle) -> (u32, u32) {
    ((h >> 32) as u32, h as u32)
}
