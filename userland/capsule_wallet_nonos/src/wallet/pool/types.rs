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

// Wire types shared across the pool/proof/assoc seams. Kept concrete (fixed
// arrays) so the contracts+prover team implements against an exact shape.

// commitment = hash(secret, nullifier, amount); stored, never sent as secret.
#[derive(Clone, Copy)]
pub struct Commitment(pub [u8; 32]);

// A locally-held shielded note. `secret`/`nullifier` never leave the capsule.
#[derive(Clone, Copy)]
pub struct Note {
    pub secret: [u8; 32],
    pub nullifier: [u8; 32],
    pub amount_wei: u128,
    pub commitment: Commitment,
}

// A merkle root (pool state root or association-set root).
#[derive(Clone, Copy)]
pub struct Root(pub [u8; 32]);

// Opaque membership proof produced by the WASM prover.
#[derive(Clone)]
pub struct Proof(pub alloc::vec::Vec<u8>);

// Reference to a submitted on-chain transaction.
#[derive(Clone, Copy)]
pub struct TxRef(pub [u8; 32]);

// Fee breakdown for a withdraw/swap, all read from the pool contract.
#[derive(Clone, Copy)]
pub struct Fees {
    pub protocol_fee_bps: u32,
    pub relayer_fee_wei: u128,
    pub gas_wei: u128,
}
