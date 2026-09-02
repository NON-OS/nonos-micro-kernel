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

use super::assembly::Assembly;
use crate::message::parse;

/// How many part-built messages may wait at once. A browser fetches
/// concurrently, so replies interleave; with a single slot two interleaved
/// messages evicted each other on every fragment and neither ever finished,
/// which read as the page hanging while frames kept arriving. The pool is
/// bounded so a flood of fabricated set ids cannot grow memory without
/// limit; past the cap the oldest part-built message is the one abandoned.
pub const MAX_PENDING: usize = 8;

/// Take one fragment, and hand back the message once it completes one.
///
/// The mixnet does not preserve order, so a fragment is placed by the
/// position in its own header rather than by when it turned up. A repeat is
/// dropped rather than counted twice, which would otherwise let a replayed
/// fragment complete a message that is still missing a piece.
pub fn collect(pending: &mut Vec<Assembly>, fragment: &[u8]) -> Option<Vec<u8>> {
    let (header, payload) = parse(fragment)?;

    let found = pending.iter().position(|a| a.holds(header.set_id) && a.total == header.total);
    let idx = match found {
        Some(i) => i,
        None => {
            if pending.len() >= MAX_PENDING {
                pending.remove(0);
            }
            pending.push(Assembly::new(header.set_id, header.total));
            pending.len() - 1
        }
    };

    let assembly = &mut pending[idx];
    let at = header.current as usize - 1;
    let place = assembly.pieces.get_mut(at)?;
    if place.is_some() {
        return None;
    }
    *place = Some(payload.to_vec());
    assembly.held += 1;

    if assembly.held < assembly.total {
        return None;
    }
    let done = pending.remove(idx);
    let mut out = Vec::new();
    for piece in done.pieces.iter() {
        out.extend_from_slice(piece.as_ref()?);
    }
    Some(out)
}
