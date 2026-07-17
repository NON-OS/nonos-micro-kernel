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

use alloc::string::String;

use crate::browser::js::token::Tok;

// Scan a `/pattern/flags` literal starting at the opening slash. A `/` inside a
// character class does not close the literal; escapes are copied verbatim.
pub fn scan_regex(cs: &[char], start: usize) -> (Tok, usize) {
    let mut i = start + 1;
    let mut pat = String::new();
    let mut in_class = false;
    while i < cs.len() {
        let c = cs[i];
        if c == '\\' && i + 1 < cs.len() {
            pat.push(c);
            pat.push(cs[i + 1]);
            i += 2;
            continue;
        }
        match c {
            '[' => in_class = true,
            ']' => in_class = false,
            '/' if !in_class => {
                i += 1;
                break;
            }
            '\n' => break,
            _ => {}
        }
        pat.push(c);
        i += 1;
    }
    let mut flags = String::new();
    while i < cs.len() && cs[i].is_ascii_alphabetic() {
        flags.push(cs[i]);
        i += 1;
    }
    (Tok::Regex(pat, flags), i)
}

// A `/` begins a regex (not division) at expression position: the start of
// input, after most punctuation, or after a keyword that expects an operand.
pub fn regex_allowed(prev: Option<&Tok>) -> bool {
    match prev {
        None => true,
        Some(Tok::Punct(p)) => p != ")" && p != "]",
        Some(Tok::Ident(k)) => matches!(
            k.as_str(),
            "return" | "typeof" | "delete" | "void" | "instanceof" | "in" | "of" | "new" | "do"
                | "else" | "case" | "await" | "yield"
        ),
        _ => false,
    }
}
