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

//! The spawn gate: what a capsule must prove before it is allowed to run.
//!
//! A capsule proves its measurement is enrolled under a policy tree the kernel
//! trusts. The vendor's tree ships in the image; a user may enrol their own so
//! software written on this machine can run on it. Both take the same proof at
//! the same strength, and the result records which of them vouched, because a
//! measurement without its authority does not say who verified it.

mod against_root;
mod error;
mod layout;
mod policy_root;
mod proved;
#[cfg(feature = "nonos-stark-attest")]
mod stark;
mod trailer;
mod verify;

pub use error::AttestError;
pub use proved::Proved;
pub use verify::verify_capsule_attestation;
