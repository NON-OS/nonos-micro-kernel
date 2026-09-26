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

//! `#!` as Linux reads it, which is not as a shell would.

use crate::exec_shebang::parse;

#[test]
fn a_bare_interpreter_has_no_argument() {
    let got = parse(b"#!/bin/sh\necho hi\n").expect("a plain shebang");
    assert_eq!(got.path, b"/bin/sh");
    assert!(got.arg.is_none());
}

#[test]
fn everything_after_the_interpreter_is_one_argument() {
    // Linux splits once.
    let got = parse(b"#!/bin/sh -e -u\n").expect("an argument follows");
    assert_eq!(got.path, b"/bin/sh");
    assert_eq!(got.arg.as_deref(), Some(&b"-e -u"[..]));
}

#[test]
fn leading_space_after_the_bang_is_skipped() {
    let got = parse(b"#!  /usr/bin/env python3\n").expect("spaces before the path");
    assert_eq!(got.path, b"/usr/bin/env");
    assert_eq!(got.arg.as_deref(), Some(&b"python3"[..]));
}

#[test]
fn a_carriage_return_does_not_become_part_of_the_path() {
    /*
     * A script edited on Windows would otherwise ask for "/bin/sh\r", which
     * resolves to nothing and reports a missing interpreter.
     */
    let got = parse(b"#!/bin/sh\r\n").expect("a CRLF script");
    assert_eq!(got.path, b"/bin/sh");
}

#[test]
fn an_elf_is_not_a_script() {
    assert!(parse(b"\x7fELF\x02\x01\x01").is_none());
}

#[test]
fn a_bang_with_nothing_after_it_is_not_an_interpreter() {
    assert!(parse(b"#!\n").is_none());
    assert!(parse(b"#!   \n").is_none());
}

#[test]
fn a_line_without_a_newline_still_parses() {
    let got = parse(b"#!/bin/sh").expect("a file that is only its shebang");
    assert_eq!(got.path, b"/bin/sh");
}
