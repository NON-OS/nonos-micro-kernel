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

// Userland thin wrappers for the eight kernel surface registry
// syscalls. Wire layout for SurfaceDescriptor and InputEvent matches
// abi/wire.toml byte for byte and is identical across every
// supported user triple.

mod attach;
mod input_drain;
mod input_post;
mod input_wait;
mod present;
mod register;
mod release;
mod share;
mod types;
mod vsync;

pub use attach::mk_surface_attach;
pub use input_drain::mk_input_event_drain;
pub use input_post::mk_input_event_post;
pub use input_wait::mk_input_event_wait;
pub use present::{mk_surface_present, mk_surface_present_rect};
pub use register::mk_surface_register;
pub use release::mk_surface_release;
pub use share::mk_surface_share;
pub use types::{
    InputEvent, SurfaceDescriptor, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_TOUCH, INPUT_KIND_WHEEL, SURFACE_FORMAT_ARGB8888,
};
pub use vsync::mk_display_vsync_wait;
