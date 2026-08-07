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

use alloc::vec;
use alloc::vec::Vec;

use super::inst::Inst;
use super::run::run;

// A successful match: the whole-match range and each capture group's range.
pub struct Match {
    pub start: usize,
    pub end: usize,
    pub groups: Vec<Option<(usize, usize)>>,
}

// Search `text` from `start` for the leftmost match of the compiled program.
pub fn search(
    prog: &[Inst],
    ngroups: usize,
    text: &[char],
    start: usize,
    ci: bool,
) -> Option<Match> {
    let nslots = 2 * (ngroups + 1);
    let mut at = start;
    while at <= text.len() {
        let mut saves = vec![usize::MAX; nslots];
        let mut steps = 0u32;
        if run(prog, 0, text, at, &mut saves, ci, &mut steps, 0) {
            let mut groups = Vec::new();
            for g in 1..=ngroups {
                let (gs, ge) = (saves[2 * g], saves[2 * g + 1]);
                let span = if gs != usize::MAX && ge != usize::MAX { Some((gs, ge)) } else { None };
                groups.push(span);
            }
            return Some(Match { start: saves[0], end: saves[1], groups });
        }
        at += 1;
    }
    None
}
