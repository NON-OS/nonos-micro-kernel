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

use super::ast::ClassItem;

pub fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

pub fn eqc(a: char, b: char, ci: bool) -> bool {
    if ci {
        a.eq_ignore_ascii_case(&b)
    } else {
        a == b
    }
}

// Whether `c` is accepted by a character class, honouring negation.
pub fn class_match(items: &[ClassItem], neg: bool, c: char, ci: bool) -> bool {
    let hit = items.iter().any(|it| item_match(it, c, ci));
    hit != neg
}

fn item_match(it: &ClassItem, c: char, ci: bool) -> bool {
    match it {
        ClassItem::Ch(x) => eqc(*x, c, ci),
        ClassItem::Range(a, b) => in_range(*a, *b, c, ci),
        ClassItem::Digit(n) => c.is_ascii_digit() != *n,
        ClassItem::Word(n) => is_word(c) != *n,
        ClassItem::Space(n) => c.is_whitespace() != *n,
    }
}

fn in_range(a: char, b: char, c: char, ci: bool) -> bool {
    if a <= c && c <= b {
        return true;
    }
    if ci {
        let lc = c.to_ascii_lowercase();
        let uc = c.to_ascii_uppercase();
        return (a <= lc && lc <= b) || (a <= uc && uc <= b);
    }
    false
}
