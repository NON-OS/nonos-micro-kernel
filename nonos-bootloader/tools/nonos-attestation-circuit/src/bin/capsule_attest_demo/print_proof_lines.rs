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

pub fn print_proof_lines(text: &str) {
    for line in text.lines() {
        if line.contains("proof_bytes")
            || line.contains("public_inputs")
            || line.contains("capsule_hash")
            || line.contains("program_hash")
            || line.contains("commitment")
        {
            println!("  {line}");
        }
    }
}
