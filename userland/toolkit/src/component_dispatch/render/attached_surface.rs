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
use nonos_libc::{mk_surface_attach, SurfaceDescriptor};
use spin::Mutex;

use super::types::AttachedSurface;

static ATTACHED_SURFACE: Mutex<Option<AttachedSurface>> = Mutex::new(None);

pub fn attached_surface(handle: u64) -> Option<SurfaceDescriptor> {
    let mut slot = ATTACHED_SURFACE.lock();
    if let Some(attached) = *slot {
        if attached.handle == handle {
            return Some(attached.desc);
        }
    }
    let mut desc = SurfaceDescriptor::default();
    let va = mk_surface_attach(handle, &mut desc);
    if va <= 0 {
        return None;
    }
    if desc.base_va == 0 {
        desc.base_va = va as u64;
    }
    *slot = Some(AttachedSurface { handle, desc });
    Some(desc)
}
