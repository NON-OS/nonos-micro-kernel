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

// Kernel-side surface registry. Owner capsules register pixel
// buffers; the kernel tracks (pid, sid) → frame list + refcount +
// metadata. Cross-address-space attach is the only path that maps
// another capsule's frames in. No userland refcount manipulation; the
// CAS lives here.

pub mod attach_map;
#[cfg(feature = "input-probe-inject")]
pub mod inject;
pub mod input_ring;
pub mod release;
pub mod share;
pub mod table;
pub mod types;
pub mod vsync;

pub use attach_map::lookup as lookup_attached_va;
pub use input_ring::{arm_input_waiter, clear_input_waiter, drain_input, input_seq, post_input};
pub use release::release_surface;
pub use share::{attach_surface, share_surface};
pub use table::{lookup_owned, register_surface};
pub use types::{
    InputEvent, RegistryError, SurfaceDescriptor, SurfaceFormat, SurfaceHandle, SurfaceSid,
    MAX_PAGES_PER_SURFACE, PIXEL_BYTES,
};
pub use vsync::{vsync_period_ns, wait_for_vsync};
