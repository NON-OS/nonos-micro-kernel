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

use super::FILLER_STEP_SIZE_INCREASE;
use alloc::vec::Vec;

/// Grow the accumulator by one slot and fold in the tail of this hop's
/// keystream. Taking the tail, not the head, is what makes the filler a mix
/// unwraps into exactly the padding the sender accounted for.
pub(super) fn filler_step(mut acc: Vec<u8>, i: usize, prng: &[u8]) -> Option<Vec<u8>> {
    if acc.len() != FILLER_STEP_SIZE_INCREASE * (i - 1) {
        return None;
    }
    let take = i.checked_mul(FILLER_STEP_SIZE_INCREASE)?;
    if take > prng.len() {
        return None;
    }
    acc.resize(acc.len() + FILLER_STEP_SIZE_INCREASE, 0);
    let tail = &prng[prng.len() - take..];
    for (a, b) in acc.iter_mut().zip(tail.iter()) {
        *a ^= *b;
    }
    Some(acc)
}
