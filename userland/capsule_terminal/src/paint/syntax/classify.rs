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

//! Reading a line and deciding what each byte is.

use super::part::{is_operator, Part};
use super::word::word_part;

/// Classify every byte of `line`.
///
/// Byte by byte rather than word by word because the caller draws by byte and
/// has to know the colour of each one, including inside a word that changes
/// meaning part way through.
pub fn classify(line: &[u8], out: &mut [Part]) {
    let mut first_word = true;
    let mut in_word = false;
    let mut quote: Option<u8> = None;
    let mut word_start = 0usize;

    for i in 0..line.len().min(out.len()) {
        let b = line[i];

        if let Some(q) = quote {
            out[i] = Part::Quoted;
            if b == q {
                quote = None;
            }
            continue;
        }

        if b == b'"' || b == b'\'' {
            quote = Some(b);
            out[i] = Part::Quoted;
            in_word = true;
            continue;
        }

        if is_operator(b) {
            out[i] = Part::Operator;
            // What follows is a new command, not an argument of the last one.
            first_word = true;
            in_word = false;
            continue;
        }

        if b == b' ' || b == b'\t' {
            out[i] = Part::Plain;
            if in_word {
                first_word = false;
            }
            in_word = false;
            continue;
        }

        if !in_word {
            in_word = true;
            word_start = i;
        }
        out[i] = word_part(line, word_start, first_word);
    }
}
