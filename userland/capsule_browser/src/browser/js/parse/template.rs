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

use alloc::boxed::Box;
use alloc::string::String;

use crate::browser::js::ast::Expr;
use crate::browser::js::lexer::tokenize;

use super::expr::expr;
use super::matching_close::matching_close;
use super::parser::Parser;

const MAX_PARTS: u32 = 64;

// Desugar a raw template body into a string concatenation chain: literal
// runs join parsed `${expr}` pieces. The leading empty string forces string
// semantics for the whole chain.
pub fn template(raw: &str) -> Expr {
    let mut out = Expr::Str(String::new());
    let mut rest = raw;
    let mut parts = 0u32;
    while !rest.is_empty() && parts < MAX_PARTS {
        parts += 1;
        match rest.find("${") {
            Some(open) => {
                if open > 0 {
                    out = Expr::Binary(
                        String::from("+"),
                        Box::new(out),
                        Box::new(Expr::Str(String::from(&rest[..open]))),
                    );
                }
                let after = &rest[open + 2..];
                let close = matching_close(after);
                let inner = &after[..close];
                let piece = expr(&mut Parser::new(tokenize(inner)));
                out = Expr::Binary(String::from("+"), Box::new(out), Box::new(piece));
                rest = after.get(close + 1..).unwrap_or("");
            }
            None => {
                out = Expr::Binary(
                    String::from("+"),
                    Box::new(out),
                    Box::new(Expr::Str(String::from(rest))),
                );
                rest = "";
            }
        }
    }
    out
}
