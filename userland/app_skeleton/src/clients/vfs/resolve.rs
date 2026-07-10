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

use crate::discover::lookup_service;

// VFS is spawned by the kernel on a fixed, well-known service port. Prefer the
// name lookup, but fall back to that port when the registry has not yet
// published `vfs_pool` (early boot, registration churn): otherwise the file
// manager and text editor report a spurious "vfs unavailable" even though the
// server is up. A genuinely dead VFS then surfaces as an IPC error on the call
// itself, which is the honest failure rather than a resolve miss.
//
// Keep in sync with `SERVICE_PORT` in src/fs/vfs_capsule/spawn.rs.
const VFS_FIXED_PORT: u32 = 4104;

/// Resolve the VFS service port, preferring the registry and falling back to
/// the fixed port. Always returns a usable port so callers never fail purely
/// because the name was not resolvable.
pub(super) fn vfs_port() -> u32 {
    match lookup_service(super::types::NAME) {
        Some(peer) if peer.port != 0 => peer.port,
        _ => VFS_FIXED_PORT,
    }
}
