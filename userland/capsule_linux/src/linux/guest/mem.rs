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

//! A guest's pages: giving it more, and reaching into the ones it has.

pub const PAGE: u64 = 4096;

/// The kernel's ceiling on one peer call. Keep in sync with MAX_SPAN in
/// src/process/foreign/peer_guard.rs.
pub const MAX_SPAN: u64 = 1 << 20;

pub fn page_down(addr: u64) -> u64 {
    addr & !(PAGE - 1)
}

/// Rounded up, saturating.
pub fn page_up(addr: u64) -> u64 {
    addr.saturating_add(PAGE - 1) & !(PAGE - 1)
}

/// The page-aligned span covering `[addr, addr + len)`, or `None` when that
/// runs past `limit`.
pub fn span_within(addr: u64, len: u64, limit: u64) -> Option<(u64, u64)> {
    let start = page_down(addr);
    let end = page_up(addr.checked_add(len)?);
    if end <= start || end > limit {
        return None;
    }
    Some((start, end - start))
}
