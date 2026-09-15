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

use super::file_kind::kind_of_name;
use super::filetype::Kind;

/// The filetype axis both Recents and Search filter on, coarsest kind last so
/// the chip row reads the same on either surface.
pub const KIND_ORDER: [Kind; 7] = [
    Kind::Dir,
    Kind::Code,
    Kind::Image,
    Kind::Doc,
    Kind::Archive,
    Kind::Exec,
    Kind::Other,
];

/// The chip label for one axis value.
pub fn kind_label(kind: Kind) -> &'static str {
    match kind {
        Kind::Dir => "Folders",
        Kind::Code => "Code",
        Kind::Image => "Images",
        Kind::Doc => "Documents",
        Kind::Archive => "Archives",
        Kind::Exec => "Programs",
        Kind::Other => "Other",
    }
}

/// Whether a path survives the active chip. `None` is the All chip.
pub fn keeps(filter: Option<Kind>, path: &str) -> bool {
    keeps_kind(filter, kind_of_name(path))
}

/// The same test against a kind the caller has already settled.
pub fn keeps_kind(filter: Option<Kind>, kind: Kind) -> bool {
    match filter {
        None => true,
        Some(want) => kind == want,
    }
}
