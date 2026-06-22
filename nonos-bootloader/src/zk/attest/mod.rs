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

mod create;
mod detect;
mod parse;
pub mod types;
mod verify;

pub use create::{calculate_proof_block_size, compute_capsule_commitment, create_zk_proof_block};
pub use detect::{find_zk_proof_offset, has_zk_proof};
pub use parse::parse_zk_proof;
pub use types::{
    BootAttestationResult, ZkProofBlock, ZK_PROOF_HEADER_SIZE, ZK_PROOF_MAGIC, ZK_PROOF_VERSION,
};
pub use verify::{verify_boot_attestation, verify_boot_attestation_with_manifest};
