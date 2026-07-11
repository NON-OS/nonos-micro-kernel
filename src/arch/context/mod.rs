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

mod facade;
mod setup;
mod tls;

#[cfg(target_arch = "x86_64")]
pub use crate::arch::x86_64::context::{SavedUser, UserEntry};

#[cfg(target_arch = "aarch64")]
pub use crate::arch::aarch64::context::{SavedUser, UserEntry};

#[cfg(target_arch = "riscv64")]
pub use crate::arch::riscv64::context::{SavedUser, UserEntry};

pub use facade::switch_to_user_pcb;
pub use setup::{setup_initial_user_pcb, SetupError};
pub use tls::set_user_tls;
