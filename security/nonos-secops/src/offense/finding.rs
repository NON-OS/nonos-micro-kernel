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

//! One attack and its verdict, with the severity of a bypass and both a human
//! and a machine rendering.

/// How bad it is if this attack is NOT refused. A gate bypass is critical; a
/// parser that can be driven to panic is a boot-time denial of service.
#[derive(Clone, Copy)]
pub enum Severity {
    Critical,
    High,
}

impl Severity {
    pub fn tag(self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
        }
    }
}

/// One attack and its verdict. `refused` true means the attestation held.
pub struct Finding {
    pub id: &'static str,
    pub category: &'static str,
    pub severity: Severity,
    pub description: &'static str,
    pub refused: bool,
}

impl Finding {
    pub fn print(&self) {
        println!(
            "  [{}] {:<9} {:<10} {}",
            if self.refused { "PASS" } else { "FAIL" },
            self.severity.tag(),
            self.id,
            self.description
        );
    }

    pub fn json(&self) -> String {
        format!(
            "{{\"id\":\"{}\",\"category\":\"{}\",\"severity\":\"{}\",\"description\":\"{}\",\"refused\":{}}}",
            self.id,
            self.category,
            self.severity.tag(),
            self.description,
            self.refused
        )
    }
}
