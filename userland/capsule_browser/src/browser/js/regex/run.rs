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

use super::classmatch::{class_match, eqc, is_word};
use super::inst::Inst;

// Backtracking bytecode matcher. `steps` caps total work (CPU) and `depth`
// caps recursion (stack); either limit reached fails the match safely.
pub fn run(
    prog: &[Inst],
    pc0: usize,
    text: &[char],
    pos0: usize,
    saves: &mut [usize],
    ci: bool,
    steps: &mut u32,
    depth: u32,
) -> bool {
    let mut pc = pc0;
    let mut pos = pos0;
    if depth > 4000 {
        return false;
    }
    loop {
        *steps += 1;
        if *steps > 1_000_000 {
            return false;
        }
        match &prog[pc] {
            Inst::Char(c) => match pos < text.len() && eqc(text[pos], *c, ci) {
                true => {
                    pc += 1;
                    pos += 1;
                }
                false => return false,
            },
            Inst::Any => match pos < text.len() && text[pos] != '\n' {
                true => {
                    pc += 1;
                    pos += 1;
                }
                false => return false,
            },
            Inst::Class(items, neg) => {
                match pos < text.len() && class_match(items, *neg, text[pos], ci) {
                    true => {
                        pc += 1;
                        pos += 1;
                    }
                    false => return false,
                }
            }
            Inst::Match => return true,
            Inst::Jmp(x) => pc = *x,
            Inst::Split(x, y) => {
                if run(prog, *x, text, pos, saves, ci, steps, depth + 1) {
                    return true;
                }
                pc = *y;
            }
            Inst::Save(n) => {
                let old = saves[*n];
                saves[*n] = pos;
                if run(prog, pc + 1, text, pos, saves, ci, steps, depth + 1) {
                    return true;
                }
                saves[*n] = old;
                return false;
            }
            Inst::AssertStart => match pos == 0 {
                true => pc += 1,
                false => return false,
            },
            Inst::AssertEnd => match pos == text.len() {
                true => pc += 1,
                false => return false,
            },
            Inst::WordB(want) => {
                let before = pos > 0 && is_word(text[pos - 1]);
                let after = pos < text.len() && is_word(text[pos]);
                if (before != after) == *want {
                    pc += 1;
                } else {
                    return false;
                }
            }
        }
    }
}
