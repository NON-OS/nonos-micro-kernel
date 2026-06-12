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

// Replace the first word of a statement with its alias expansion, keeping
// the remaining arguments. Expansion is single-level (an alias body is
// not itself re-aliased), which keeps it simple and loop-free.
pub fn alias_expand(stmt: &[u8], aliases: &[(Vec<u8>, Vec<u8>)]) -> Vec<u8> {
    let mut s = 0;
    while s < stmt.len() && stmt[s] == b' ' {
        s += 1;
    }
    let mut e = s;
    while e < stmt.len() && stmt[e] != b' ' {
        e += 1;
    }
    match aliases.iter().find(|(k, _)| k.as_slice() == &stmt[s..e]) {
        Some((_, exp)) => {
            let mut out = Vec::with_capacity(exp.len() + stmt.len() - e);
            out.extend_from_slice(exp);
            out.extend_from_slice(&stmt[e..]);
            out
        }
        None => stmt.to_vec(),
    }
}
