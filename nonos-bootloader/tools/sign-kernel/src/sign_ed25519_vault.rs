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
use ed25519_dalek::VerifyingKey;

use crate::args::Args;
use crate::ed25519_result::Ed25519Result;
use crate::vault::{sign_kernel_with_vault, VaultClient};

pub fn sign_ed25519_vault(args: &Args, message: &[u8]) -> Result<Ed25519Result> {
    let vault_addr = args
        .vault_addr
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--vault-addr is required"))?;
    let vault_token = args
        .vault_token
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("VAULT_TOKEN required when using --vault-addr"))?;
    println!("Signing via HashiCorp Vault...");
    println!("  Vault address: {}", vault_addr);
    println!("  Key name: {}", args.vault_key_name);
    println!();
    let client = VaultClient::new(vault_addr.clone(), vault_token.clone(), None)
        .map_err(|e| anyhow::anyhow!("vault connection failed: {}", e))?;
    let pubkey_bytes = client
        .get_transit_public_key(&args.vault_key_name)
        .map_err(|e| anyhow::anyhow!("failed to get public key: {}", e))?;
    let verifying_key = VerifyingKey::from_bytes(&pubkey_bytes)
        .map_err(|e| anyhow::anyhow!("invalid public key: {}", e))?;
    println!("Public Key (hex): {}", hex::encode(verifying_key.as_bytes()));
    let pk_hash = blake3::hash(verifying_key.as_bytes());
    println!("Public Key BLAKE3: {}", pk_hash.to_hex());
    println!();
    let sig = sign_kernel_with_vault(vault_addr, vault_token, &args.vault_key_name, message)
        .map_err(|e| anyhow::anyhow!("vault signing failed: {}", e))?;
    Ok(Ed25519Result {
        sig,
        verifying_key,
    })
}
