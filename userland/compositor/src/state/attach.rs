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

use nonos_libc::{mk_surface_attach, mk_surface_release, SurfaceDescriptor};

use crate::debug;
use crate::sw_blitter::Surface;

pub const MAX_ATTACH: usize = 32;
const EMPTY: Surface = Surface { base_va: 0, stride: 0, width: 0, height: 0, byte_len: 0 };

#[derive(Clone, Copy, Default)]
struct Slot {
    handle: u64,
    surface: Surface,
    in_use: bool,
}

pub struct AttachCache {
    slots: [Slot; MAX_ATTACH],
}

impl AttachCache {
    pub const fn new() -> Self {
        Self { slots: [Slot { handle: 0, surface: EMPTY, in_use: false }; MAX_ATTACH] }
    }
    pub fn get_or_attach(&mut self, handle: u64) -> Option<Surface> {
        if handle == 0 {
            return None;
        }
        if let Some(s) = self.slots.iter().find(|s| s.in_use && s.handle == handle) {
            return Some(s.surface);
        }
        let mut desc = SurfaceDescriptor::default();
        debug::marker(b"attach call");
        let rc = mk_surface_attach(handle, &mut desc);
        if rc <= 0 {
            return None;
        }
        debug::marker(b"attach ok");
        let surface =
            Surface { base_va: rc as u64, stride: desc.stride, width: desc.width, height: desc.height, byte_len: desc.byte_len };
        if let Some(slot) = self.slots.iter_mut().find(|s| !s.in_use) {
            *slot = Slot { handle, surface, in_use: true };
        }
        Some(surface)
    }
    pub fn forget(&mut self, handle: u64) -> Result<(), ()> {
        for slot in self.slots.iter_mut() {
            if slot.in_use && slot.handle == handle {
                if mk_surface_release(handle) < 0 {
                    return Err(());
                }
                *slot = Slot::default();
            }
        }
        Ok(())
    }
}
