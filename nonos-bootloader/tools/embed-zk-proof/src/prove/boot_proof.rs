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

use anyhow::Result;

use crate::Args;

use super::boot_proof_runtime::runtime_boot_proof;
use super::boot_proof_static::static_boot_proof;
use super::types::TransparentBootProof;

pub fn create_transparent_boot_proof(
    args: &Args,
    kernel_hash: &[u8; 32],
) -> Result<TransparentBootProof> {
    if args.sidecar {
        runtime_boot_proof(args, kernel_hash)
    } else {
        static_boot_proof(args, kernel_hash)
    }
}
