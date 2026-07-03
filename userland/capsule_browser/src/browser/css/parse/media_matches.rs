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

use super::media_feature::feature_matches;

// Evaluate an @media condition. A comma-separated list is an OR of
// independent queries; within one query every feature must hold.
pub(super) fn media_matches(cond: &str, viewport_w: u32) -> bool {
    let cond = cond.trim().to_ascii_lowercase();
    cond.split(',').any(|q| query_matches(q, viewport_w))
}

fn query_matches(q: &str, viewport_w: u32) -> bool {
    let mut q = q.trim();
    let mut invert = false;
    if let Some(r) = q.strip_prefix("not ") {
        invert = true;
        q = r.trim();
    } else if let Some(r) = q.strip_prefix("only ") {
        q = r.trim();
    }
    let mut ok = true;
    // Media type: this is a screen render; print and speech never apply.
    let head = q.split(['(', ' ']).next().unwrap_or("");
    if head == "print" || head == "speech" {
        ok = false;
    }
    let mut rest = q;
    while ok {
        let Some(open) = rest.find('(') else { break };
        let after = &rest[open + 1..];
        let Some(close) = after.find(')') else { break };
        if !feature_matches(&after[..close], viewport_w) {
            ok = false;
        }
        rest = after.get(close + 1..).unwrap_or("");
    }
    ok != invert
}
