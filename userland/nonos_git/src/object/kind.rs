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

//! The four kinds of object git stores.

/// Tags here are the annotated-tag object, not the lightweight ref.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl ObjectKind {
    /// The ASCII name git writes into the object header.
    pub const fn name(self) -> &'static [u8] {
        match self {
            ObjectKind::Blob => b"blob",
            ObjectKind::Tree => b"tree",
            ObjectKind::Commit => b"commit",
            ObjectKind::Tag => b"tag",
        }
    }

    /// Parse the header name, rejecting anything git would not write.
    pub fn from_name(name: &[u8]) -> Option<ObjectKind> {
        match name {
            b"blob" => Some(ObjectKind::Blob),
            b"tree" => Some(ObjectKind::Tree),
            b"commit" => Some(ObjectKind::Commit),
            b"tag" => Some(ObjectKind::Tag),
            _ => None,
        }
    }
}
