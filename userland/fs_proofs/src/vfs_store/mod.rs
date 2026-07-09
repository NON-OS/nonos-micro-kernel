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

// The real vfs store logic, assembled from the capsule source via #[path] with
// a host clock shim in place of the syscall one. Seeding (`seed`, `packages`)
// is left out because it needs baked-in signed artifacts; proofs build their
// own fixtures on an empty store.

mod time;

#[path = "../../../capsule_vfs/src/store/fdtable/chmod.rs"]
mod chmod;
#[path = "../../../capsule_vfs/src/store/fdtable/close.rs"]
mod close;
#[path = "../../../capsule_vfs/src/store/fdtable/copy.rs"]
mod copy;
#[path = "../../../capsule_vfs/src/store/fdtable/lookup.rs"]
mod lookup;
#[path = "../../../capsule_vfs/src/store/fdtable/mkdir.rs"]
mod mkdir;
#[path = "../../../capsule_vfs/src/store/fdtable/new.rs"]
mod new;
#[path = "../../../capsule_vfs/src/store/fdtable/open.rs"]
mod open;
#[path = "../../../capsule_vfs/src/store/fdtable/query.rs"]
mod query;
#[path = "../../../capsule_vfs/src/store/fdtable/read.rs"]
mod read;
#[path = "../../../capsule_vfs/src/store/fdtable/rename.rs"]
mod rename;
#[path = "../../../capsule_vfs/src/store/fdtable/rmdir.rs"]
mod rmdir;
#[path = "../../../capsule_vfs/src/store/fdtable/truncate.rs"]
mod truncate;
#[path = "../../../capsule_vfs/src/store/fdtable/types.rs"]
mod types;
#[path = "../../../capsule_vfs/src/store/fdtable/unlink.rs"]
mod unlink;
#[path = "../../../capsule_vfs/src/store/fdtable/usage.rs"]
mod usage;
#[path = "../../../capsule_vfs/src/store/fdtable/write.rs"]
mod write;
#[path = "../../../capsule_vfs/src/store/fdtable/zeroize.rs"]
mod zeroize;

pub use types::{Store, StoreError};
