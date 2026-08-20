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

//! Signing authorities a user enrolled on this machine, so software written
//! here can run here.
//!
//! Without this, building software on NØNOS is impossible: the spawn gate
//! refuses anything it cannot prove, and a capsule compiled minutes ago was
//! never enrolled under the vendor's policy tree. The answer is not to weaken
//! the gate. It is to let the machine hold more than one authority and to say
//! which one vouched for what.
//!
//! Two properties hold. The proof is identical either way, so locally built
//! code is verified exactly as rigorously as shipped code. And the authority
//! is carried through to attestation, so a remote party is never told that
//! something built on this laptop was signed by the project.
//!
//! Enrolment does not survive a reboot. On a system that keeps nothing, a
//! signing authority is not an exception.

mod authority;
mod consent;
mod enrol;
mod error;
mod pending;
mod resolve;
mod table;

pub use authority::Authority;
pub use enrol::{confirm_dev_root, dev_root_count, request_dev_root};
pub use error::EnrolError;
pub use resolve::{authority_for, enrolled_roots};
pub use table::MAX_DEV_ROOTS;
