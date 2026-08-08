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

/// Take one fragment, and hand back the message once it completes one.
///
/// The mixnet does not preserve order, so a fragment is placed by the
/// position in its own header rather than by when it turned up. A repeat is
/// dropped rather than counted twice, which would otherwise let a replayed
/// fragment complete a message that is still missing a piece.
pub fn collect(slot: &mut Option<Assembly>, fragment: &[u8]) -> Option<Vec<u8>> {
    let (header, payload) = parse(fragment)?;

    // A fragment of a different message means the one in progress will never
    // complete, so the newer one takes the slot rather than being dropped.
    let start_over = match slot {
        Some(assembly) => !assembly.holds(header.set_id) || assembly.total != header.total,
        None => true,
    };
    if start_over {
        *slot = Some(Assembly::new(header.set_id, header.total));
    }

    let assembly = slot.as_mut()?;
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
    let mut out = Vec::new();
    for piece in assembly.pieces.iter() {
        out.extend_from_slice(piece.as_ref()?);
    }
    *slot = None;
    Some(out)
}
