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
//! The two pieces smart HTTP adds around the plain git commands.

/// The service banner a smart HTTP advertisement opens with.
pub(super) fn banner(service: &str) -> Vec<u8> {
    let line = format!("# service={service}\n");
    let mut out = format!("{:04x}", line.len() + 4).into_bytes();
    out.extend_from_slice(line.as_bytes());
    out.extend_from_slice(b"0000");
    out
}

/// Which git command a request path names.
pub(super) fn service_of(path: &str) -> Option<&'static str> {
    if path.contains("git-receive-pack") {
        Some("git-receive-pack")
    } else if path.contains("git-upload-pack") {
        Some("git-upload-pack")
    } else {
        None
    }
}
