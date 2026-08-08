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

//! Host-runnable proofs binding kernel mechanisms to the properties their Lean
//! models state. The real Rust of each mechanism is pulled in via `#[path]` and
//! run directly, so the property is proven of the code the kernel executes.
//! Modules land here as their Lean model moves from a specification to a
//! code-bound proof; see `verification/lean/REFINEMENT.md`.

pub mod bounds;
pub mod buddy;
pub mod compositor;
pub mod constants;
pub mod context;
pub mod heap;
pub mod iommu;
pub mod mmio;
pub mod nonce;
pub mod phys;
pub mod quota;
pub mod refcount;
pub mod region;
pub mod ring;
pub mod scheduler;
pub mod spawn;
pub mod spec;
pub mod timer;

#[cfg(test)]
mod compositor_tests;

#[cfg(test)]
mod constants_tests;

#[cfg(test)]
mod heap_tests;

#[cfg(test)]
mod iommu_tests;

#[cfg(test)]
mod refinement_tests;

#[cfg(kani)]
mod kani_proofs;
