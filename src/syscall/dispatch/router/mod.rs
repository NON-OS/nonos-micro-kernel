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

mod admin_ops;
mod crypto;
mod dispatch_fn;
mod entry;
mod graphics_backend;
mod graphics_present;
mod input_ops;
mod microkernel_ops;
mod surface_handlers;
mod surface_ops;

#[cfg(feature = "nonos-user-entry-proof")]
mod unknown_diag;

pub(crate) use entry::handle_syscall_dispatch;
