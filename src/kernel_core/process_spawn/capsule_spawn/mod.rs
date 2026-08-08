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
mod instance;
mod runner;
mod spec;

pub(crate) use attested_parent::AttestedParent;
#[cfg(feature = "nonos-dev-unverified-capsules")]
pub use runner::spawn;
pub use runner::spawn_verified;
pub(crate) use runner::spawn_verified_as;
pub(crate) use runner::{classify_tier, Tier};
#[cfg(feature = "nonos-dev-unverified-capsules")]
pub use spec::CapsuleSpec;
pub use spec::{CapsuleSpecVerified, SpawnError};
// Runtime capsule loading from the VFS store, driven by the install syscall.
pub(crate) use from_vfs::load_capsule_from_vfs;
pub(crate) use from_vfs::validity_now_ms;
pub use from_vfs::{CapsuleArtifacts, LoadError};
// Spawning an extra window instance of an embedded, attested app capsule.
pub use instance::{spawn_next as spawn_next_instance, InstanceEndpoint, InstanceSpawn};
