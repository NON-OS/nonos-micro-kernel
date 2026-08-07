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
//! The seam between the protocol and whatever carries it.

extern crate alloc;

use alloc::vec::Vec;

use super::error::TransportError;

/// A request and response pair against one remote repository.
///
/// `path` is appended to the repository URL the transport was opened with, so
/// an implementation holds the host and the base path and this layer only
/// names the service it wants.
pub trait Transport {
    /// Issue a GET and return the body.
    fn get(&mut self, path: &str) -> Result<Vec<u8>, TransportError>;

    /// Issue a POST with `content_type` and return the body.
    fn post(
        &mut self,
        path: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, TransportError>;
}
