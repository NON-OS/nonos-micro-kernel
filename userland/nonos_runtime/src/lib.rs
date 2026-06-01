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

extern crate alloc;
extern crate nonos_panic;

mod boot;
mod entropy;
mod exit;
mod hooks;
mod macros;
pub mod prelude;
mod run;
mod time;
mod yield_now;

pub use boot::boot;
pub use entropy::entropy;
pub use exit::exit;
pub use hooks::on_cleanup;
pub use nonos_abi::{
    input_drain, mmap, munmap, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_KEY_DOWN, INPUT_KIND_KEY_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_TOUCH, INPUT_KIND_WHEEL,
};
pub use run::run;
pub use time::time_millis;
pub use yield_now::yield_now;

pub use nonos_cap as cap;
pub use nonos_ipc as ipc;
pub use nonos_log as log;
pub use nonos_service as service;
pub use nonos_surface as surface;
