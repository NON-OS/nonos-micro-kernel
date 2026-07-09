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

mod attested_parent;
mod from_vfs;
mod runner;
mod spec;

pub(crate) use attested_parent::AttestedParent;
#[cfg(feature = "nonos-dev-unverified-capsules")]
pub use runner::spawn;
pub use runner::spawn_verified;
pub(crate) use runner::spawn_verified_as;
#[cfg(feature = "nonos-dev-unverified-capsules")]
pub use spec::CapsuleSpec;
pub use spec::{CapsuleSpecVerified, SpawnError};
// Runtime capsule loading from the VFS store, driven by the install syscall.
pub use from_vfs::{CapsuleArtifacts, LoadError};
pub(crate) use from_vfs::load_capsule_from_vfs;
