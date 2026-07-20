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

//! Drawing the out-of-domain point for the Poseidon-committed prover and verifier.

use super::super::field::{Fp, Fp2};
use super::super::poseidon_transcript::PoseidonTranscript;

/// The out-of-domain point from the Poseidon transcript: off the coset and off the
/// trace domain, so every DEEP and periodic denominator is invertible. Both prover
/// and verifier run this identically, so they agree on the point.
pub(super) fn draw_ood_point_poseidon(
    transcript: &mut PoseidonTranscript,
    shift: Fp,
    n: usize,
    t: usize,
) -> Fp2 {
    let shift_n = Fp2::from_base(shift.pow(n as u64));
    let mut z = transcript.challenge_fp2();
    while z.pow(n as u64) == shift_n || z.pow(t as u64) == Fp2::ONE {
        z = transcript.challenge_fp2();
    }
    z
}
