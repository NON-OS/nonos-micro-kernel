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
//! What a host and a path may contain.

/// A host is letters, digits, dots and hyphens.
///
/// A colon is refused rather than ignored. A caller that always connects on
/// one port and quietly disregards a port somebody wrote would connect
/// somewhere they did not ask for.
pub(super) fn is_host_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'.' || b == b'-'
}

/// A path is printable ASCII without space.
///
/// That covers every character a git forge url uses, and excludes everything
/// that could end a header line or start another. A url carrying a carriage
/// return would otherwise let whoever supplied it append headers of their
/// own, or split one request into two.
pub(super) fn is_path_byte(b: u8) -> bool {
    (0x21..=0x7E).contains(&b)
}
