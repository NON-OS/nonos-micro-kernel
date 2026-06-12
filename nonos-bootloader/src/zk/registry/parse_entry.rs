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

use super::types_category::CircuitCategory;
use super::types_entry::DynamicCircuitEntry;

pub fn parse_single_entry(
    section: &[u8],
    offset: &mut usize,
) -> Result<DynamicCircuitEntry, &'static str> {
    if *offset + 48 > section.len() {
        return Err("circuit: truncated entry");
    }
    let mut program_hash = [0u8; 32];
    program_hash.copy_from_slice(&section[*offset..*offset + 32]);
    *offset += 32;
    let permissions = u32::from_le_bytes(
        section[*offset..*offset + 4]
            .try_into()
            .map_err(|_| "circuit: permissions parse failed")?,
    );
    *offset += 4;
    let category = match section[*offset] {
        0 => CircuitCategory::System,
        1 => CircuitCategory::Community,
        2 => CircuitCategory::User,
        _ => return Err("circuit: invalid category"),
    };
    *offset += 1;
    let name_len = section[*offset] as usize;
    *offset += 1;
    let version_len = section[*offset] as usize;
    *offset += 2;
    let vk_offset = u32::from_le_bytes(
        section[*offset..*offset + 4].try_into().map_err(|_| "circuit: vk_offset parse failed")?,
    ) as usize;
    *offset += 4;
    let vk_len = u32::from_le_bytes(
        section[*offset..*offset + 4].try_into().map_err(|_| "circuit: vk_len parse failed")?,
    ) as usize;
    *offset += 4;
    if *offset + name_len + version_len > section.len() {
        return Err("circuit: truncated strings");
    }
    let name = section[*offset..*offset + name_len].to_vec();
    *offset += name_len + version_len;
    if vk_offset + vk_len > section.len() {
        return Err("circuit: VK out of bounds");
    }
    let vk_bytes = section[vk_offset..vk_offset + vk_len].to_vec();
    Ok(DynamicCircuitEntry { program_hash, vk_bytes, name, permissions, category, loaded_at: 0 })
}
