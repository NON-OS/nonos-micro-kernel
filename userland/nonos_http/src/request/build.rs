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
//! What a request is made of.

extern crate alloc;

use alloc::vec::Vec;

/// A request ready to be written to a stream.
pub struct Request {
    pub bytes: Vec<u8>,
}

/// Collected before the bytes are laid out, so the content length is known
/// by the time the headers are written.
pub struct RequestBuilder<'a> {
    pub(super) method: &'a str,
    pub(super) target: &'a str,
    pub(super) host: &'a str,
    pub(super) user_agent: &'a str,
    pub(super) accept: &'a str,
    pub(super) content_type: Option<&'a str>,
    pub(super) body: &'a [u8],
}
