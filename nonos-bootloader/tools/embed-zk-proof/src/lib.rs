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

pub mod args;
pub mod embed;
pub mod footer;
pub mod kernel;
pub mod prove;

pub use args::Args;
pub use embed::{
    assemble_attested_image, create_zk_block, AttestedImage, ZkBlockParams, ZK_PROOF_MAGIC,
};
pub use footer::{create_image_footer, FOOTER_MAGIC, FOOTER_SIZE};
pub use kernel::{
    compute_capsule_commitment, compute_kernel_hash, load_signed_kernel, SignedKernel,
};
pub use prove::{create_circuit_params, extract_public_inputs, generate_proof, load_proving_key};
