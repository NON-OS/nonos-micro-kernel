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

pub fn validate_address(address: &str) -> Result<(), String> {
    if address.len() != 42 || !address.starts_with("0x") {
        return Err("contributor must be a 20-byte 0x address".into());
    }
    if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("contributor address contains non-hex characters".into());
    }
    Ok(())
}

pub fn address_bytes(address: &str) -> Result<[u8; 20], String> {
    validate_address(address)?;
    let raw = hex::decode(&address[2..]).map_err(|e| e.to_string())?;
    let mut out = [0u8; 20];
    out.copy_from_slice(&raw);
    Ok(out)
}
