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

//! Host-runnable proofs for the kernel synchronisation primitives. The real
//! permit arithmetic of the counting semaphore and the sequence discipline of
//! the seqlock are pulled in via `#[path]` and run directly, so the properties
//! the Lean `Nonos.Semaphore` and `Nonos.Seqlock` models state are proven
//! about the code the kernel actually executes.

pub mod semaphore;
pub mod seqlock;
pub mod spec;

#[cfg(test)]
mod refinement_tests;

#[cfg(kani)]
mod kani_proofs;
