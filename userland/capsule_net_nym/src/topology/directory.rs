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

use alloc::vec::Vec;

use super::types::Node;

/// Where a directory came from, which decides how it earns trust. Fetched
/// bytes need a signature; a table compiled into the image is already covered
/// by the image's own signatures and STARK enrollment.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Provenance {
    /// Compiled into the attested image.
    Image,
    /// Received over the network, signed by a trusted authority.
    Signed,
    /// Fetched over TLS from a named API. The chain proves who answered, not
    /// that an operator vouched for the answer, so it is kept apart from
    /// `Signed` rather than folded into it.
    Fetched,
}

#[derive(Clone, Copy)]
pub struct DirectoryMeta {
    pub epoch: u64,
    pub not_before_ms: u64,
    pub not_after_ms: u64,
    pub issuer: [u8; 32],
    pub provenance: Provenance,
}

pub struct ParsedDirectory {
    pub meta: DirectoryMeta,
    pub nodes: Vec<Node>,
}
