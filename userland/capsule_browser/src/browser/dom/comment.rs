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

use core::iter::Peekable;
use core::str::CharIndices;

/// Skip a comment if one starts here, and say whether it did.
///
/// A comment ends at `-->`, not at the first `>`. Reading to the first `>`
/// stops inside any comment that contains one, and commented out markup
/// contains one by definition. Everything after that `>` is then read as
/// document content, so a page shows the markup its author had disabled,
/// closes elements that were never opened, and ends the comment's text in
/// the middle of a sentence.
///
/// The caller has already taken the `<`, so this looks at what follows it
/// and only commits if the three characters are there.
pub fn skip_comment(chars: &mut Peekable<CharIndices>) -> bool {
    let mut look = chars.clone();
    for want in ['!', '-', '-'] {
        if look.next().map(|(_, c)| c) != Some(want) {
            return false;
        }
    }
    *chars = look;

    // Two dashes and a close, with any run of dashes before it accepted, so
    // that a comment ending `--->` closes here rather than running to the end
    // of the document.
    let mut dashes = 0u32;
    for (_, c) in chars.by_ref() {
        match c {
            '-' => dashes += 1,
            '>' if dashes >= 2 => return true,
            _ => dashes = 0,
        }
    }
    // Unterminated. Everything left was inside the comment, which is what a
    // browser does with one rather than showing the rest of the file.
    true
}
