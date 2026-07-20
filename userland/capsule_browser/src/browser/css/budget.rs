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

// The rule index prunes candidates per node, so real pages stay far below
// this; the cap only stops pathological sheets (huge universal buckets over
// huge trees) from going quadratic. Budget exhaustion degrades styling from
// the end of the document instead of dropping rules for the whole page.
const MAX_SELECTOR_CHECKS: usize = 16_000_000;

// Total author-rule candidate tests allowed for one cascade. UA rules are
// never budgeted: base block layout must survive a hostile author sheet.
pub(super) struct MatchBudget {
    left: usize,
}

impl MatchBudget {
    pub(super) fn new() -> Self {
        MatchBudget { left: MAX_SELECTOR_CHECKS }
    }

    // Reserve n candidate tests. False once dry, at which point the caller
    // skips author matching and keeps the inherited + UA style.
    pub(super) fn take(&mut self, n: usize) -> bool {
        if self.left < n {
            self.left = 0;
            return false;
        }
        self.left -= n;
        true
    }
}
