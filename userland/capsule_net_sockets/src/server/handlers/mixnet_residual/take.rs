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

use super::types::RESIDUAL;
use crate::sockets::SocketKey;

/// Serve a read from what a previous frame left over.
///
/// Returns how many bytes were copied, which is zero when nothing is held.
pub fn take(key: SocketKey, out: &mut [u8]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut slots = RESIDUAL.lock();
    let Some(slot) = slots.iter_mut().find(|s| s.holds(key)) else {
        return 0;
    };
    let n = (slot.len - slot.off).min(out.len());
    out[..n].copy_from_slice(&slot.buf[slot.off..slot.off + n]);
    slot.off += n;
    if slot.off >= slot.len {
        slot.len = 0;
        slot.off = 0;
    }
    n
}
