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

use uefi::prelude::*;

use crate::boot::zk_challenge::current_timestamp_secs;
use crate::zk::{verify_proof_binding, ZkProofBlock, ZkPublicInputs};

pub fn verify_boot_proof_binding(
    st: &SystemTable<Boot>,
    kernel_hash: &[u8; 32],
    proof_block: &ZkProofBlock,
) -> Result<(), &'static str> {
    if !proof_block.is_runtime() {
        return Ok(());
    }
    let inputs = ZkPublicInputs::from_bytes(&proof_block.public_inputs)
        .ok_or("ZK public inputs malformed")?;
    verify_proof_binding(&inputs, kernel_hash, current_timestamp_secs(st))
        .map_err(|_| "ZK replay binding failed")
}
