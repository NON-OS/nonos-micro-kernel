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
//! Pushing a branch to a remote.

extern crate alloc;

use crate::oid::ObjectId;
use crate::pack::write_pack;
use crate::repo::objects_to_send;
use crate::storage::Storage;
use crate::transport::{Transport, TransportError};
use crate::wire::{push_request, RefUpdate};

use super::discover::{discover, RECEIVE_PACK};
use super::report::accepted;

/// Send everything the remote needs to move `name` to `head`.
///
/// The value the remote currently holds is read from its own advertisement
/// rather than assumed, so a push that would overwrite work which arrived in
/// the meantime is refused by the receiver instead of silently applied.
pub fn push<T: Transport, S: Storage>(
    transport: &mut T,
    storage: &S,
    git_dir: &str,
    head: &ObjectId,
    name: &str,
) -> Result<(), TransportError> {
    let refs = discover(transport, RECEIVE_PACK)?;
    let old = refs.iter().find(|r| r.name == name).map(|r| r.id).unwrap_or_else(ObjectId::zero);
    if old == *head {
        return Ok(());
    }

    let have: alloc::vec::Vec<ObjectId> = refs.iter().map(|r| r.id).collect();
    let objects =
        objects_to_send(storage, git_dir, head, &have).map_err(|_| TransportError::Malformed)?;
    let pack = write_pack(&objects);

    let update = RefUpdate { old, new: *head, name };
    let mut path = alloc::string::String::from("/");
    path.push_str(RECEIVE_PACK);
    let body = push_request(&[update], &pack);
    let response = transport.post(&path, "application/x-git-receive-pack-request", &body)?;
    accepted(&response, name)
}
