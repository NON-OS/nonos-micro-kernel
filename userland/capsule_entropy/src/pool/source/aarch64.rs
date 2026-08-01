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

/// Draw from the kernel's generator.
///
/// There is no unprivileged equivalent of RDRAND to reach for here. FEAT_RNG
/// adds RNDR, but it is optional and absent on the Apple M-series among
/// others, and whether it exists is only readable from EL1, so a capsule
/// cannot check before issuing the instruction and would take an undefined
/// instruction trap where it is missing. The kernel can check, and already
/// backs this call with RNDR or the virtio entropy device as the board
/// allows, so the source decision belongs there.
pub(in crate::pool) unsafe fn fill(out: &mut [u8]) -> bool {
    nonos_libc::crypto_random(out.as_mut_ptr(), out.len()) == out.len() as i64
}
