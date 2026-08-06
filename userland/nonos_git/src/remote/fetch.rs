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
//! Fetching objects from a remote.

extern crate alloc;

use alloc::vec::Vec;

use crate::oid::ObjectId;
use crate::transport::{Transport, TransportError};
use crate::wire::want_request;

use super::discover::UPLOAD_PACK;
use super::pack_body::pack_body;

/// Ask for everything reachable from `wants`, to `depth` commits deep, and
/// return the pack the remote sends back.
///
/// A depth of zero asks for the whole history. Anything else is a shallow
/// fetch, and the caller has to record that in the repository or git will
/// treat the missing parents as damage.
pub fn fetch<T: Transport>(
    transport: &mut T,
    wants: &[ObjectId],
    depth: u32,
) -> Result<Vec<u8>, TransportError> {
    if wants.is_empty() {
        return Err(TransportError::Malformed);
    }
    let body = want_request(wants, depth);
    let mut path = alloc::string::String::from("/");
    path.push_str(UPLOAD_PACK);
    let response = transport.post(&path, "application/x-git-upload-pack-request", &body)?;
    Ok(pack_body(&response)?.to_vec())
}
