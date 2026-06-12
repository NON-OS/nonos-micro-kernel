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

use serde_json::Value;

pub fn text<'a>(receipt: &'a Value, name: &str) -> Result<&'a str, String> {
    receipt
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("receipt is missing field {name}"))
}

pub fn number(receipt: &Value, name: &str) -> Result<u64, String> {
    receipt
        .get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("receipt is missing field {name}"))
}

pub fn expect(name: &str, got: &str, want: &str) -> Result<(), String> {
    if got == want {
        Ok(())
    } else {
        Err(format!("{name} mismatch: receipt {got}, recomputed {want}"))
    }
}
