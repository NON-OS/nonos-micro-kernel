// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::state::ATTESTATION_STATE;
use crate::security::attestation::quote::AttestationQuote;

pub fn generate_attestation_quote(nonce: [u8; 32], timestamp: u64) -> AttestationQuote {
    let state = ATTESTATION_STATE.lock();
    state.generate_quote(nonce, timestamp)
}

pub fn generate_signed_quote_with_aik(
    nonce: [u8; 32],
    timestamp: u64,
    aik: &ed25519_dalek::SigningKey,
) -> AttestationQuote {
    let state = ATTESTATION_STATE.lock();
    state.generate_signed_quote(nonce, timestamp, aik)
}

pub fn verify_attestation_quote(
    quote: &AttestationQuote,
    attestation_public_key: &[u8; 32],
) -> bool {
    quote.verify(attestation_public_key)
}
