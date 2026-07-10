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

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use super::theme::{
    FOREGROUND, SYN_COMMENT, SYN_FUNCTION, SYN_KEYWORD, SYN_NUMBER, SYN_STRING, SYN_TYPE,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Tok {
    Text,
    Keyword,
    String,
    Comment,
    Number,
    Type,
    Function,
}

pub fn color(t: Tok) -> u32 {
    match t {
        Tok::Text => FOREGROUND,
        Tok::Keyword => SYN_KEYWORD,
        Tok::String => SYN_STRING,
        Tok::Comment => SYN_COMMENT,
        Tok::Function => SYN_FUNCTION,
        Tok::Number => SYN_NUMBER,
        Tok::Type => SYN_TYPE,
    }
}

// A single generic lexer that covers the common surface of most languages:
// `//`, `#` and `--` line comments, `/* */` block comments, `"` and `'`
// strings with escapes, numbers, and a broad keyword union. Good enough to make
// any source file read like code without a per-language grammar.
pub fn classify(buf: &[u8]) -> Vec<Tok> {
    let n = buf.len();
    let mut out = vec![Tok::Text; n];
    let mut i = 0;
    while i < n {
        let c = buf[i];
        // line comments: // , # , --
        let line_comment = (c == b'/' && buf.get(i + 1) == Some(&b'/'))
            || c == b'#'
            || (c == b'-' && buf.get(i + 1) == Some(&b'-'));
        if line_comment {
            let mut j = i;
            while j < n && buf[j] != b'\n' {
                out[j] = Tok::Comment;
                j += 1;
            }
            i = j;
            continue;
        }
        // block comment /* ... */
        if c == b'/' && buf.get(i + 1) == Some(&b'*') {
            let mut j = i + 2;
            while j + 1 < n && !(buf[j] == b'*' && buf[j + 1] == b'/') {
                j += 1;
            }
            let end = (j + 2).min(n);
            out[i..end].fill(Tok::Comment);
            i = end;
            continue;
        }
        // strings " ... " or ' ... '
        if c == b'"' || c == b'\'' {
            let q = c;
            let mut j = i + 1;
            while j < n && buf[j] != q && buf[j] != b'\n' {
                if buf[j] == b'\\' {
                    j += 1;
                }
                j += 1;
            }
            let end = (j + 1).min(n);
            out[i..end].fill(Tok::String);
            i = end;
            continue;
        }
        // numbers
        if c.is_ascii_digit() {
            let mut j = i;
            while j < n && (buf[j].is_ascii_alphanumeric() || buf[j] == b'.' || buf[j] == b'_') {
                out[j] = Tok::Number;
                j += 1;
            }
            i = j;
            continue;
        }
        // identifiers / keywords / types
        if c.is_ascii_alphabetic() || c == b'_' {
            let mut j = i;
            while j < n && (buf[j].is_ascii_alphanumeric() || buf[j] == b'_') {
                j += 1;
            }
            let word = &buf[i..j];
            let tok = if is_keyword(word) {
                Tok::Keyword
            } else if word[0].is_ascii_uppercase() {
                Tok::Type
            } else if buf.get(j) == Some(&b'(') {
                // An identifier immediately before a paren is a call.
                Tok::Function
            } else {
                Tok::Text
            };
            out[i..j].fill(tok);
            i = j;
            continue;
        }
        i += 1;
    }
    out
}

// Union of keywords across Rust, C/C++, JS/TS, Python, Go, shell and a few
// more. Broad on purpose: highlighting a keyword that a given language does not
// use is harmless; missing common ones is what looks amateur.
fn is_keyword(w: &[u8]) -> bool {
    matches!(
        w,
        b"fn"
            | b"let"
            | b"mut"
            | b"const"
            | b"static"
            | b"pub"
            | b"use"
            | b"mod"
            | b"struct"
            | b"enum"
            | b"trait"
            | b"impl"
            | b"where"
            | b"match"
            | b"as"
            | b"dyn"
            | b"move"
            | b"ref"
            | b"crate"
            | b"super"
            | b"self"
            | b"Self"
            | b"if"
            | b"else"
            | b"for"
            | b"while"
            | b"loop"
            | b"break"
            | b"continue"
            | b"return"
            | b"in"
            | b"do"
            | b"switch"
            | b"case"
            | b"default"
            | b"goto"
            | b"def"
            | b"class"
            | b"lambda"
            | b"pass"
            | b"yield"
            | b"with"
            | b"try"
            | b"except"
            | b"finally"
            | b"raise"
            | b"import"
            | b"from"
            | b"global"
            | b"nonlocal"
            | b"async"
            | b"await"
            | b"function"
            | b"var"
            | b"new"
            | b"delete"
            | b"typeof"
            | b"instanceof"
            | b"extends"
            | b"implements"
            | b"interface"
            | b"type"
            | b"public"
            | b"private"
            | b"protected"
            | b"package"
            | b"void"
            | b"int"
            | b"char"
            | b"long"
            | b"short"
            | b"float"
            | b"double"
            | b"bool"
            | b"unsigned"
            | b"signed"
            | b"sizeof"
            | b"typedef"
            | b"union"
            | b"volatile"
            | b"register"
            | b"extern"
            | b"inline"
            | b"go"
            | b"func"
            | b"defer"
            | b"chan"
            | b"map"
            | b"range"
            | b"select"
            | b"throw"
            | b"catch"
            | b"finally_"
            | b"true"
            | b"false"
            | b"null"
            | b"nil"
            | b"None"
            | b"True"
            | b"False"
            | b"and"
            | b"or"
            | b"not"
            | b"is"
            | b"then"
            | b"fi"
            | b"elif"
            | b"echo"
            | b"export"
            | b"local"
            | b"unset"
            | b"end"
    )
}
