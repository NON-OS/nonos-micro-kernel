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

use core::marker::PhantomData;

use ark_ff::PrimeField;

use crate::constants::PCR_PREIMAGE_LEN;

#[derive(Clone)]
pub struct NonosAttestationCircuit<F: PrimeField> {
    pub capsule_hash_hi: Option<F>,
    pub capsule_hash_lo: Option<F>,
    pub program_hash_hi: Option<F>,
    pub program_hash_lo: Option<F>,
    pub capability_mask: Option<F>,
    pub commitment_hi: Option<F>,
    pub commitment_lo: Option<F>,
    pub pcr_preimage: Option<[u8; PCR_PREIMAGE_LEN]>,
    pub hardware_attestation: Option<u64>,
    pub phantom: PhantomData<F>,
}
