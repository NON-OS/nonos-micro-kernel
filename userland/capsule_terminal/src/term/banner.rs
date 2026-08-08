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

use crate::term::scrollback::Scrollback;

pub fn paint_motd(sb: &mut Scrollback) {
    sb.push_line(b"");
    sb.push_line("  \u{00D8} NONOS shell".as_bytes());
    sb.push_line(b"");
    sb.push_line(b"  RAM-ephemeral microkernel. Capsules, not processes.");
    sb.push_line(b"  Hybrid Ed25519 + ML-DSA-65 trust chain end to end.");
    sb.push_line(b"  Not Linux. Not BSD. Native.");
    sb.push_line(b"");
    sb.push_line(b"  type 'nox help' for the command index.");
    sb.push_line(b"");
}
