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

//! Restoring a decision made on an earlier boot.

use nonos_app_skeleton::clients::vfs;
use nonos_libc::{mk_getpid, mk_local_restore};

/// Where the token that restores the decision is kept. It opens nothing on
/// another machine or under another kernel, so it may sit on the disk.
pub(super) const TOKEN: &[u8] = b"/nonos/consent/local.token";

/// What the disk says about an earlier decision.
pub enum Restore {
    /// True when there was one and the kernel took it back.
    Known(bool),
    /// vfs has not finished loading the disk, so no token yet proves nothing.
    NotYet,
}

/// Settled is read before the file: if staging ends between the two, the
/// read already saw the loaded store, so an absent token is really absent.
pub fn restore() -> Restore {
    let settled = !matches!(vfs::store_settled(), Ok(false));
    let raw = match vfs::read_file(mk_getpid(), TOKEN, 32) {
        Ok(raw) => raw,
        Err(_) if !settled => return Restore::NotYet,
        Err(_) => return Restore::Known(false),
    };
    let Ok(token) = <[u8; 32]>::try_from(raw.as_slice()) else {
        return Restore::Known(false);
    };
    Restore::Known(mk_local_restore(&token) == 0)
}
