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

mod cpl;
mod dump_gdt;
mod dump_trap;
mod print_hex;
#[cfg(feature = "nonos-user-entry-proof")]
mod user_proof;

pub use dump_gdt::dump_gdt;
pub use dump_trap::dump_trap;
#[cfg(feature = "nonos-trap-kstack-writer")]
pub use dump_trap::dump_user_around_rsp;
pub use print_hex::print_hex_u64;
#[cfg(feature = "nonos-user-entry-proof")]
pub use user_proof::assert_user_entry;
