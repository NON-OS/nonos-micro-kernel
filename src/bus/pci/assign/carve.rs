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

/// Take `size` bytes from a window, at the alignment the bus requires.
///
/// A memory BAR decodes an address whose low bits are fixed at zero, so the
/// base has to be a multiple of the size or the device answers on an address
/// nobody assigned it. Returns the base and where the next allocation starts,
/// or `None` when the window cannot hold it.
///
/// Kept pure and separate from the window state so the alignment and bounds
/// arguments can be proven rather than argued: see the `pci_window` harnesses
/// in `userland/kernel_proofs`.
pub fn carve(cursor: u64, limit: u64, size: u64) -> Option<(u64, u64)> {
    if size == 0 || !size.is_power_of_two() {
        return None;
    }
    let base = cursor.checked_add(size - 1)? & !(size - 1);
    let next = base.checked_add(size)?;
    if next > limit {
        return None;
    }
    Some((base, next))
}
