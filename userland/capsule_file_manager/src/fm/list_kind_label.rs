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

use super::filetype::Kind;

/// The human name for a file kind, for the list's Type column and the grid
/// card's meta line. The classification itself stays in `file_kind`; this is
/// only how it is spelled on screen.
pub fn kind_label(kind: Kind) -> &'static str {
    match kind {
        Kind::Dir => "Folder",
        Kind::Code => "Code",
        Kind::Image => "Image",
        Kind::Doc => "Document",
        Kind::Archive => "Archive",
        Kind::Exec => "Binary",
        Kind::Other => "File",
    }
}
