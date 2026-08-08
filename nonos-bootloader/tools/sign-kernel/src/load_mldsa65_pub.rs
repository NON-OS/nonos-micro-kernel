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

use anyhow::{bail, Result};
use nonos_capsule_sign::algs::AlgId;
use nonos_capsule_sign::keys::read_pub;

use crate::args::Args;

pub fn load_mldsa65_pub(args: &Args) -> Result<Vec<u8>> {
    let path = args.mldsa65_pub.as_ref().ok_or_else(|| {
        anyhow::anyhow!("--mldsa65-pub is required for kernel signing")
    })?;
    let key = read_pub(path).map_err(|e| anyhow::anyhow!("{}", e))?;
    if key.alg != AlgId::MlDsa65 {
        bail!("ML-DSA-65 pubkey file has wrong algorithm");
    }
    // `key` now owns zeroize-on-drop material, so its bytes cannot be moved
    // out. Clone them; the original is wiped when `key` drops.
    Ok(key.bytes.clone())
}
