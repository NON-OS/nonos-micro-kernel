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

//! Host-runnable proofs for kernel isolation and authorization. The real page
//! permission and user-copy bounds source is pulled in via `#[path]` and run
//! directly, so the invariants are proven about the code that actually gates
//! memory access.

pub mod arch;
pub mod bus;
pub mod capabilities;
pub mod elf;
pub mod memory;
pub mod syscall;
pub mod time;
pub mod spec;
pub mod usercopy;

#[cfg(test)]
mod authorization_tests;
#[cfg(test)]
mod elf_tests;
#[cfg(test)]
mod permissions_tests;
#[cfg(test)]
mod syscall_tests;
#[cfg(test)]
mod refinement_tests;
#[cfg(test)]
mod usercopy_tests;

#[cfg(kani)]
mod kani_proofs;
