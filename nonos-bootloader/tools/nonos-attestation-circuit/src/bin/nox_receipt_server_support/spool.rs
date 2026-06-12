// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

use std::path::{Path, PathBuf};

use serde_json::Value;

use nonos_attestation_circuit::nox::validate_address;

pub fn spool_paths(spool: &Path, receipt: &Value) -> Result<(PathBuf, PathBuf), String> {
    let kind = field(receipt, "kind")?;
    let contributor = field(receipt, "contributor_address")?;
    validate_address(&contributor)?;
    let epoch = receipt
        .get("claim_epoch")
        .or_else(|| receipt.get("round"))
        .and_then(Value::as_u64)
        .ok_or("receipt has neither claim_epoch nor round")?;
    if !kind.bytes().all(|b| b.is_ascii_uppercase() || b == b'_') {
        return Err("kind contains unexpected characters".into());
    }
    let dir = spool.join(format!("epoch-{epoch}")).join(contributor.to_ascii_lowercase());
    Ok((dir.join(format!("{kind}.json")), dir.join(format!("{kind}.artifact"))))
}

fn field(receipt: &Value, name: &str) -> Result<String, String> {
    receipt
        .get(name)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("receipt is missing field {name}"))
}
