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

//! Splitting the node records out of a directory answer.

use alloc::vec::Vec;

use super::data_span::array_span;
use super::objects::objects;

/// The key the node records sit under.
const NODES_KEY: &str = "data";

/// The node records in a directory answer.
///
/// The answer is a document, not a list: the records sit under a key beside
/// a status block and a pagination block. Splitting the document at its top
/// level yields exactly one object, the document itself, and therefore no
/// nodes, which reads downstream as an empty directory rather than as a
/// document that was never opened.
///
/// A bare array is still read, by falling back to the whole text, because
/// that is what the answer used to be and a reader that only understands the
/// newer shape breaks on the older one.
pub fn node_objects(text: &[u8], cap: usize) -> Vec<&[u8]> {
    match array_span(text, NODES_KEY) {
        Some(list) => objects(list, cap),
        None => objects(text, cap),
    }
}
