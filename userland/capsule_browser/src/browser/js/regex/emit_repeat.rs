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

use alloc::vec::Vec;

use super::ast::Re;
use super::compile::emit;
use super::inst::Inst;

const CAP: usize = 1000;

// Expand `inner{min,max}` into mandatory copies followed by a greedy star (open
// upper bound) or a run of optional copies, backpatching skip targets.
pub(super) fn emit_repeat(
    prog: &mut Vec<Inst>,
    inner: &Re,
    min: usize,
    max: Option<usize>,
    greedy: bool,
) {
    let min = min.min(CAP);
    for _ in 0..min {
        emit(prog, inner);
    }
    match max {
        None => {
            let l = prog.len();
            prog.push(Inst::Split(0, 0));
            let body = prog.len();
            emit(prog, inner);
            prog.push(Inst::Jmp(l));
            let end = prog.len();
            prog[l] = if greedy { Inst::Split(body, end) } else { Inst::Split(end, body) };
        }
        Some(mx) => {
            let opt = mx.min(CAP).saturating_sub(min);
            let mut splits = Vec::new();
            for _ in 0..opt {
                splits.push(prog.len());
                prog.push(Inst::Split(0, 0));
                emit(prog, inner);
            }
            let end = prog.len();
            for s in splits {
                let body = s + 1;
                prog[s] = if greedy { Inst::Split(body, end) } else { Inst::Split(end, body) };
            }
        }
    }
}
