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
//! Asking a remote what it has.

extern crate alloc;

use alloc::vec::Vec;

use crate::transport::{Transport, TransportError};
use crate::wire::{parse_advertisement, RemoteRef};

/// The service name a fetch advertises under.
pub(super) const UPLOAD_PACK: &str = "git-upload-pack";
/// The service name a push advertises under.
pub(super) const RECEIVE_PACK: &str = "git-receive-pack";

/// Read the remote's ref advertisement for `service`.
///
/// This is the first request of any exchange. It is also the only one that
/// tells us what the remote holds, which is what a push needs in order to
/// name the value it expects a ref to have.
pub fn discover<T: Transport>(
    transport: &mut T,
    service: &str,
) -> Result<Vec<RemoteRef>, TransportError> {
    let mut path = alloc::string::String::from("/info/refs?service=");
    path.push_str(service);
    let body = transport.get(&path)?;
    parse_advertisement(&body).map_err(|_| TransportError::Malformed)
}
