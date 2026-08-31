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

//! What is running right now, and what each program proved before it started.
//!
//! Boot measurements say what the machine loaded. They say nothing about a
//! capsule spawned an hour later. This registry is the runtime half: every
//! capsule that passes the spawn gate is recorded with the measurement its
//! proof was checked against, and removed when it exits. `registry_root`
//! folds that set into one digest, which is the value an attestation signs.

mod complete;
mod entry;
mod record;
mod root;
mod table;

pub use complete::registry_complete;
pub use entry::AttestedCapsule;
pub use record::{forget_attested, record_attested};
pub use root::{attested_count, registry_root};
