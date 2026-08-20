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

//! The one parameter set every attestation prover and verifier reads.
//!
//! These were five separate copies: the kernel's capsule gate, the kernel's
//! self-attestation, the bootloader's pre-jump check, the enrolment tool, and
//! the proof-of-concept test. Prover and verifier must agree exactly or no
//! proof verifies, so a drift upward is loud. A drift downward in queries or
//! grinding is not: proofs keep verifying, at less security than anyone reading
//! the other four files believes.
//!
//! Changing a value here changes what every existing trailer verifies against,
//! so every capsule must be re-enrolled after.

/// Rounds in the attestation AIR's hash, as `2^LOG_ROUNDS`. This is the width-8
/// Poseidon in `air::poseidon`, not the width-12 one the Merkle tree and FRI use.
///
/// All 32 are full rounds. The published schedule for this field and S-box is 8
/// full plus 22 partial; a partial round differs only in the S-box covering one
/// lane instead of eight, so 32 full clears 30 mixed. Width and constants still
/// differ from the published set, so this defends the round margin, not the
/// provenance. Costs `2^(LOG_ROUNDS + log_slots)` trace rows.
pub const LOG_ROUNDS: u32 = 5;

/// FRI query count. Each query is an independent chance to catch a prover that
/// committed to something other than a low-degree polynomial.
pub const N_QUERIES: usize = 32;

/// Proof-of-work bits on the transcript, raising the cost of grinding for a
/// favourable challenge.
pub const GRIND_BITS: u32 = 16;

/// Extra blowup beyond the AIR's own, trading proof size for soundness.
pub const EXTRA_BLOWUP_BITS: u32 = 3;

/// Rounds actually run, for a caller that wants the count rather than its log.
pub const fn rounds() -> usize {
    1usize << LOG_ROUNDS
}
