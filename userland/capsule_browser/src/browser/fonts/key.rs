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

// Key for a font family name: the FNV-1a hash of its unquoted, lowercased
// form. The computed style carries the key rather than the name so it stays
// Copy; the registry maps keys to loaded faces. Zero means "no custom face"
// and never collides because FNV-1a of a non-empty name is never zero here.
pub fn family_key(name: &str) -> u32 {
    let trimmed = name.trim().trim_matches('"').trim_matches('\'').trim();
    if trimmed.is_empty() {
        return 0;
    }
    let mut h: u32 = 0x811c_9dc5;
    for b in trimmed.bytes() {
        let lower = b.to_ascii_lowercase();
        h ^= lower as u32;
        h = h.wrapping_mul(0x0100_0193);
    }
    h.max(1)
}
