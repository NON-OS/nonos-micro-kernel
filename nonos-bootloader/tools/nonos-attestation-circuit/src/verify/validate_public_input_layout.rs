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

pub(super) fn validate_public_input_layout(bytes: &[u8]) -> Result<(), String> {
    let ranges = [
        (0usize, 16usize, "capsule hash high prefix"),
        (32, 48, "capsule hash low prefix"),
        (96, 120, "policy epoch prefix"),
        (128, 152, "capability mask prefix"),
        (160, 176, "commitment high prefix"),
        (192, 208, "commitment low prefix"),
    ];
    for (start, end, label) in ranges {
        if bytes[start..end].iter().any(|byte| *byte != 0) {
            return Err(format!("{label} must be zero"));
        }
    }
    Ok(())
}
