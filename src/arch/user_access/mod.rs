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

//! The window in which the kernel is allowed to touch user memory.
//!
//! Both architectures default to refusing: x86_64 through SMAP, which faults
//! on a privileged access to a user page unless `AC` is set, and aarch64
//! through PAN, which does the same unless `PSTATE.PAN` is clear. The point of
//! both is that a kernel bug which follows an attacker-supplied pointer faults
//! instead of succeeding, so the window has to be narrow and explicit.
//!
//! [`with_user_access`] is the only way to open it. Nothing exposes the raw
//! set and clear, because a path that opened the window and returned early
//! would leave the whole kernel able to read userspace by accident.

mod scope;

#[cfg(target_arch = "aarch64")]
mod pan;
#[cfg(target_arch = "x86_64")]
mod smap;

pub use scope::with_user_access;
