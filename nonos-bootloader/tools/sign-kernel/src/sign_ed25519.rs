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

use std::fs;

use anyhow::{bail, Context, Result};
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};

use crate::args::Args;
use crate::ed25519_result::Ed25519Result;
use crate::sign_ed25519_vault::sign_ed25519_vault;

pub fn sign_ed25519(args: &Args, message: &[u8]) -> Result<Ed25519Result> {
    if args.vault_addr.is_some() {
        return sign_ed25519_vault(args, message);
    }
    let key_path = args
        .key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Must provide either --key or --vault-addr"))?;
    let key_bytes = fs::read(key_path)
        .with_context(|| format!("Failed to read key file: {}", key_path.display()))?;
    if key_bytes.len() != 32 {
        bail!(
            "Key file must be exactly 32 bytes (Ed25519 seed), got {} bytes",
            key_bytes.len()
        );
    }
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&key_bytes);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key: VerifyingKey = (&signing_key).into();
    println!("Public Key (hex): {}", hex::encode(verifying_key.as_bytes()));
    let pk_hash = blake3::hash(verifying_key.as_bytes());
    println!("Public Key BLAKE3: {}", pk_hash.to_hex());
    println!();
    println!("Signing kernel with Ed25519 (local key)...");
    let signature = signing_key.sign(message);
    Ok(Ed25519Result {
        sig: signature.to_bytes(),
        verifying_key,
    })
}
