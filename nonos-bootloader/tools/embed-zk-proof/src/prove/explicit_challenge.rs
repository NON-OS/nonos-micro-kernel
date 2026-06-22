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

use super::hex32::hex32;

pub fn explicit_challenge(args: &Args) -> Result<([u8; 32], [u8; 32], u64)> {
    let boot_nonce = hex32(
        "boot_nonce",
        args.boot_nonce.as_deref().ok_or_else(|| anyhow::anyhow!("boot_nonce required"))?,
    )?;
    let machine_id = hex32(
        "machine_id",
        args.machine_id.as_deref().ok_or_else(|| anyhow::anyhow!("machine_id required"))?,
    )?;
    let timestamp = args.timestamp.ok_or_else(|| anyhow::anyhow!("timestamp required"))?;
    Ok((boot_nonce, machine_id, timestamp))
}
