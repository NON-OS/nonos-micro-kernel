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

use ed25519_dalek::VerifyingKey;

use crate::args::Args;

pub fn print_summary(args: &Args, verifying_key: &VerifyingKey) {
    println!();
    println!("=== Summary ===");
    println!("Input:     {}", args.input.display());
    println!("Output:    {}", args.output.display());
    if let Some(ref key_path) = args.key {
        println!("Key:       {}", key_path.display());
    } else if let Some(ref vault_addr) = args.vault_addr {
        println!("Vault:     {} (key: {})", vault_addr, args.vault_key_name);
    }
    println!("Signature: Ed25519 + ML-DSA-65");
    println!();
    println!("IMPORTANT: Embed this public key in the bootloader:");
    println!();
    let pk = verifying_key.as_bytes();
    println!("pub const NONOS_SIGNING_KEY: &[u8; 32] = &[");
    for chunk in pk.chunks(8) {
        let hex_line: Vec<String> = chunk.iter().map(|b| format!("0x{:02x}", b)).collect();
        println!("    {},", hex_line.join(", "));
    }
    println!("];");
}
