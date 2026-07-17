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

// Emit alternation as a chain of splits: each alternative but the last is
// guarded by a Split that falls through to the next, then jumps to the end.
pub(super) fn emit_alt(prog: &mut Vec<Inst>, alts: &[Re]) {
    let mut jmp_ends = Vec::new();
    for (k, alt) in alts.iter().enumerate() {
        if k == alts.len() - 1 {
            emit(prog, alt);
            break;
        }
        let split_at = prog.len();
        prog.push(Inst::Split(0, 0));
        let body = prog.len();
        emit(prog, alt);
        jmp_ends.push(prog.len());
        prog.push(Inst::Jmp(0));
        let next = prog.len();
        prog[split_at] = Inst::Split(body, next);
    }
    let end = prog.len();
    for j in jmp_ends {
        prog[j] = Inst::Jmp(end);
    }
}
