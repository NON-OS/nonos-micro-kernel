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

// CR4 and the AC flag are deliberately absent. SMEP, SMAP and UMIP go on once
// at boot from `memory::mmu`, which reads back what the part accepted, and the
// AC window belongs to `arch::user_access`, which is the only code that knows
// when it is legitimately open. Bare `stac`/`clac` used to be exported here
// with no caller, which is an invitation to open that window without the
// guard that closes it.
mod segment_base;
mod start;
mod switch;
mod user_access;

pub use segment_base::{read_fs_base, write_fs_base, write_gs_base, write_kernel_gs_base};
pub use start::switch_to_new_thread;
pub use switch::switch_context;
pub use user_access::with_user_access;
