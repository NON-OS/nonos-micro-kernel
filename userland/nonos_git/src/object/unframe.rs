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

//! Reading framed bytes back into a kind and its content.

use super::decimal;
use super::kind::ObjectKind;

/// Split framed bytes into kind and content, validating the header. `None` if
/// the header is malformed or the stated length disagrees with the content, so
/// a corrupt object is never read as valid.
pub fn unframe(framed: &[u8]) -> Option<(ObjectKind, &[u8])> {
    let space = framed.iter().position(|b| *b == b' ')?;
    let nul = framed.iter().position(|b| *b == 0)?;
    if nul < space {
        return None;
    }
    let kind = ObjectKind::from_name(&framed[..space])?;
    let size = decimal::parse(&framed[space + 1..nul])?;
    let content = &framed[nul + 1..];
    if content.len() as u64 != size {
        return None;
    }
    Some((kind, content))
}
