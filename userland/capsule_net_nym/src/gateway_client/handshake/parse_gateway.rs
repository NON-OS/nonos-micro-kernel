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

use super::material::Material;
use super::sizes::{EPHEMERAL_BYTES, GATEWAY_MATERIAL_BYTES};

/// Split the gateway's reply into its ephemeral key and sealed material.
pub fn parse_gateway_material(bytes: &[u8]) -> Option<([u8; EPHEMERAL_BYTES], Material)> {
    if bytes.len() != GATEWAY_MATERIAL_BYTES {
        return None;
    }
    let mut ephemeral = [0u8; EPHEMERAL_BYTES];
    ephemeral.copy_from_slice(&bytes[..EPHEMERAL_BYTES]);
    let material = Material::from_bytes(&bytes[EPHEMERAL_BYTES..])?;
    Some((ephemeral, material))
}
