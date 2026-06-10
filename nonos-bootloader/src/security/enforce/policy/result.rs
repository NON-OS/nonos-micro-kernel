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

use super::types::SecurityPolicy;

#[derive(Debug)]
pub struct EnforcementResult {
    pub allow_boot: bool,
    pub reason: &'static str,
    pub warnings: [Option<&'static str>; 8],
    pub warning_count: usize,
    pub policy: SecurityPolicy,
}

impl EnforcementResult {
    pub fn new(policy: SecurityPolicy) -> Self {
        Self {
            allow_boot: true,
            reason: "checks passed",
            warnings: [None; 8],
            warning_count: 0,
            policy,
        }
    }

    pub fn deny(&mut self, reason: &'static str) {
        self.allow_boot = false;
        self.reason = reason;
    }

    pub fn warn(&mut self, warning: &'static str) {
        if self.warning_count < 8 {
            self.warnings[self.warning_count] = Some(warning);
            self.warning_count += 1;
        }
    }
}
