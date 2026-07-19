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

//! The lexer: source bytes to a token stream. proeve source is ASCII; a non-ASCII
//! or otherwise unexpected byte is a typed error carrying its offset. Line
//! comments run from `//` to end of line.

use alloc::string::String;
use alloc::vec::Vec;

use super::CompileError;

// One lexical token.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tok {
    Let,
    Assert,
    Inv,
    Sel,
    Ident(String),
    Num(u64),
    Plus,
    Minus,
    Star,
    Assign,
    EqEq,
    LParen,
    RParen,
    Comma,
    Semi,
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Tokenize `src`, or report the first byte that begins no valid token.
pub fn lex(src: &str) -> Result<Vec<Tok>, CompileError> {
    let b = src.as_bytes();
    let mut toks: Vec<Tok> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let ch = b[i];
        if ch.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if ch == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            i += 2;
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if is_ident_start(ch) {
            let start = i;
            while i < b.len() && is_ident_continue(b[i]) {
                i += 1;
            }
            let word = &src[start..i];
            toks.push(match word {
                "let" => Tok::Let,
                "assert" => Tok::Assert,
                "inv" => Tok::Inv,
                "sel" => Tok::Sel,
                _ => Tok::Ident(String::from(word)),
            });
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            let mut value: u64 = 0;
            while i < b.len() && b[i].is_ascii_digit() {
                let digit = (b[i] - b'0') as u64;
                value = match value.checked_mul(10).and_then(|v| v.checked_add(digit)) {
                    Some(v) => v,
                    None => return Err(CompileError::NumberTooLarge { at: start }),
                };
                i += 1;
            }
            toks.push(Tok::Num(value));
            continue;
        }
        let single = match ch {
            b'+' => Some(Tok::Plus),
            b'-' => Some(Tok::Minus),
            b'*' => Some(Tok::Star),
            b'(' => Some(Tok::LParen),
            b')' => Some(Tok::RParen),
            b',' => Some(Tok::Comma),
            b';' => Some(Tok::Semi),
            _ => None,
        };
        if let Some(tok) = single {
            toks.push(tok);
            i += 1;
            continue;
        }
        if ch == b'=' {
            if i + 1 < b.len() && b[i + 1] == b'=' {
                toks.push(Tok::EqEq);
                i += 2;
            } else {
                toks.push(Tok::Assign);
                i += 1;
            }
            continue;
        }
        return Err(CompileError::UnexpectedChar { at: i });
    }
    Ok(toks)
}
