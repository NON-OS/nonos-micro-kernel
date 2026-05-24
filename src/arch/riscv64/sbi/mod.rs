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

pub mod base;
pub mod error;
pub mod extensions;
pub mod hart;
pub mod ipi;
pub mod rfence;
pub mod timer;

pub use base::{impl_id, impl_version, sbi_call, sbi_version};
pub use error::SbiError;
pub use extensions::{probe_extension, Extension};
pub use hart::{hart_get_status, hart_start, hart_stop, hart_suspend};
pub use ipi::send_ipi;
pub use timer::set_timer;

mod console;
mod reset;

pub use console::{console_getchar, console_putchar};
pub use reset::{shutdown, system_reset};
