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

#![no_std]
#![no_main]

mod fill;

use crate::fill::fill;
use nonos_runtime::prelude::*;

const CAPS: u64 = cap::CAP_CORE_EXEC
    | cap::CAP_MEMORY
    | cap::CAP_GRAPHICS_DISPLAY_QUERY
    | cap::CAP_GRAPHICS_SURFACE_CREATE
    | cap::CAP_GRAPHICS_SURFACE_MAP
    | cap::CAP_GRAPHICS_PRESENT;
const W: u32 = 256;
const H: u32 = 256;

fn main() {
    let desc = surface::SurfaceDescriptor {
        width: W,
        height: H,
        stride: W * 4,
        format: surface::SURFACE_FORMAT_ARGB8888,
        byte_len: (W * H * 4) as u64,
        base_va: 0,
        flags: 0,
    };
    let handle = surface::create(&desc);
    if handle < 0 {
        exit(1);
    }
    let h = handle as u64;
    let mut mapped = surface::SurfaceDescriptor::default();
    if surface::attach(h, &mut mapped) < 0 {
        exit(1);
    }
    fill(&mapped, 0xFF1E_1E2E);
    let _ = surface::damage(h);
    loop {
        yield_now();
    }
}

nonos_main!(CAPS, main);
