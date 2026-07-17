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
use super::emit_alt::emit_alt;
use super::emit_repeat::emit_repeat;
use super::inst::Inst;

// Compile a regex tree into bytecode, wrapping the whole match in save slots
// 0 and 1 and terminating with Match.
pub fn compile(re: &Re) -> Vec<Inst> {
    let mut prog = Vec::new();
    prog.push(Inst::Save(0));
    emit(&mut prog, re);
    prog.push(Inst::Save(1));
    prog.push(Inst::Match);
    prog
}

pub(super) fn emit(prog: &mut Vec<Inst>, re: &Re) {
    match re {
        Re::Char(c) => prog.push(Inst::Char(*c)),
        Re::Any => prog.push(Inst::Any),
        Re::Class(items, neg) => prog.push(Inst::Class(items.clone(), *neg)),
        Re::Start => prog.push(Inst::AssertStart),
        Re::End => prog.push(Inst::AssertEnd),
        Re::WordB(w) => prog.push(Inst::WordB(*w)),
        Re::Concat(v) => {
            for r in v {
                emit(prog, r);
            }
        }
        Re::Group(inner, idx) => {
            if let Some(i) = idx {
                prog.push(Inst::Save(2 * i));
            }
            emit(prog, inner);
            if let Some(i) = idx {
                prog.push(Inst::Save(2 * i + 1));
            }
        }
        Re::Alt(v) => emit_alt(prog, v),
        Re::Repeat(inner, min, max, greedy) => emit_repeat(prog, inner, *min, *max, *greedy),
    }
}
