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

pub mod capabilities;
pub mod elf;
pub mod memory;
pub mod syscall;
pub mod time;
pub mod spec;
pub mod usercopy;

// The PID selection arithmetic is dependency-free, so the real source is pulled
// in directly and its invariants (never 0, wraps to 1, skips active, unique)
// are proven on the host.
#[path = "../../../src/process/core/table/pid_alloc.rs"]
pub mod pid_alloc;

// Reserved core-service name/port predicate: dependency-free, so the real
// source is pulled in and its coverage of every core name/port is proven.
#[path = "../../../src/services/registry/reserved.rs"]
pub mod reserved;

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
#[cfg(test)]
mod pid_tests;
#[cfg(test)]
mod reserved_tests;

#[cfg(kani)]
mod kani_proofs;
