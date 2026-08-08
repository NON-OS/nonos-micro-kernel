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

/// Bytes a fragment header takes when its set stands alone.
pub const UNLINKED_HEADER_LEN: usize = 7;

/// Set ids are carried with the top bit set. The receiver reads that bit to
/// tell a fragment header apart from the older unfragmented form, so it is
/// part of the encoding rather than part of the id.
pub(super) const SET_ID_MARKER: i32 = 1 << 31;

/// The most fragments one set can be split into.
pub const MAX_FRAGMENTS: u8 = 255;

/// One piece of a message, headed so the far end can put the pieces back in
/// order.
///
/// Only the unlinked form is produced here. Linking exists to chain sets past
/// 255 fragments, and a request that large does not arise from a proxied
/// connection whose writes are already split well below that.
pub struct Fragment {
    pub set_id: i32,
    pub total: u8,
    pub current: u8,
}

impl Fragment {
    /// Serialize the header and append `payload`.
    pub fn into_bytes(self, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(UNLINKED_HEADER_LEN + payload.len());
        out.extend_from_slice(&(self.set_id | SET_ID_MARKER).to_be_bytes());
        out.push(self.total);
        out.push(self.current);
        // Zero here says the set is not linked to another. A non zero value
        // would be read as four more bytes of linked set id.
        out.push(0);
        out.extend_from_slice(payload);
        out
    }
}
