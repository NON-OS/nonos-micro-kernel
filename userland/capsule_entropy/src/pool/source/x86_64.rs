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

/// Draw from RDRAND, the CPU's on-die conditioned generator.
///
/// RDRAND can legitimately come up empty when the hardware queue is drained,
/// so a handful of retries is expected; giving up after that is reported to
/// the caller rather than papered over with a weaker source.
#[target_feature(enable = "rdrand")]
pub(in crate::pool) unsafe fn fill(out: &mut [u8]) -> bool {
    let mut filled = 0;
    while filled < out.len() {
        let mut word: u64 = 0;
        let mut tries = 0;
        while core::arch::x86_64::_rdrand64_step(&mut word) != 1 {
            tries += 1;
            if tries >= 32 {
                return false;
            }
        }
        let take = core::cmp::min(8, out.len() - filled);
        out[filled..filled + take].copy_from_slice(&word.to_le_bytes()[..take]);
        filled += take;
    }
    true
}
