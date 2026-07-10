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

//! Prefix a body with our own pid so the server's anti-impersonation check
//! accepts the request, followed by the length-prefixed path.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

pub(super) fn owner_body(tail: &[u8]) -> Vec<u8> {
    let pid = mk_getpid();
    let mut body = Vec::with_capacity(5 + tail.len());
    body.extend_from_slice(&pid.to_le_bytes());
    body.push(tail.len() as u8);
    body.extend_from_slice(tail);
    body
}
