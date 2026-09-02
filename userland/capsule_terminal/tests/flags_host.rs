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

//! Host tests for the shared short-flag parser.

extern crate alloc;

#[path = "../src/command/flags"]
mod flags {
    pub mod cluster;
    pub mod err;
    pub mod num;
    pub mod parse;
    pub mod spec;
}

use flags::num::parse_usize;
use flags::parse::parse;
use flags::spec::Spec;

fn rm_spec() -> Spec<'static> {
    Spec::new(b"rm", b"rf")
}

fn head_spec() -> Spec<'static> {
    Spec::new(b"head", b"q").valued(b"n").numeric(b'n')
}

#[test]
fn no_flags_leaves_every_arg_an_operand() {
    let args: [&[u8]; 2] = [b"a.txt", b"b.txt"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert_eq!(p.operands, [b"a.txt", b"b.txt"]);
    assert!(!p.has(b'r'));
}

#[test]
fn clustered_flags_split_into_each_letter() {
    let args: [&[u8]; 2] = [b"-rf", b"x"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r') && p.has(b'f'));
    assert_eq!(p.operands, [b"x"]);
}

#[test]
fn flags_are_accepted_after_operands() {
    let args: [&[u8]; 3] = [b"x", b"-r", b"-f"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r') && p.has(b'f'));
    assert_eq!(p.operands, [b"x"]);
}

#[test]
fn double_dash_ends_flag_parsing() {
    let args: [&[u8]; 3] = [b"-r", b"--", b"-f"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert!(p.has(b'r'));
    assert!(!p.has(b'f'));
    assert_eq!(p.operands, [b"-f"]);
}

#[test]
fn a_bare_dash_is_an_operand() {
    let args: [&[u8]; 1] = [b"-"];
    let p = parse(&rm_spec(), &args).unwrap();
    assert_eq!(p.operands, [b"-"]);
}

#[test]
fn unknown_flag_is_an_error_not_an_operand() {
    let args: [&[u8]; 1] = [b"-z"];
    let err = parse(&rm_spec(), &args).unwrap_err();
    assert_eq!(err, b"rm: unknown flag -z".to_vec());
}

#[test]
fn unknown_letter_inside_a_cluster_is_an_error() {
    let args: [&[u8]; 1] = [b"-rz"];
    let err = parse(&rm_spec(), &args).unwrap_err();
    assert_eq!(err, b"rm: unknown flag -z".to_vec());
}

#[test]
fn separate_value_flag_takes_the_next_argument() {
    let args: [&[u8]; 3] = [b"-n", b"5", b"f.txt"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert_eq!(p.operands, [b"f.txt"]);
}

#[test]
fn attached_value_flag_takes_the_cluster_remainder() {
    let args: [&[u8]; 1] = [b"-n5"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert!(p.operands.is_empty());
}

#[test]
fn numeric_shorthand_is_the_same_as_the_count_flag() {
    let args: [&[u8]; 2] = [b"-5", b"f.txt"];
    let p = parse(&head_spec(), &args).unwrap();
    assert_eq!(p.value(b'n'), Some(&b"5"[..]));
    assert_eq!(p.operands, [b"f.txt"]);
}

#[test]
fn a_value_flag_may_trail_a_boolean_in_one_cluster() {
    let args: [&[u8]; 2] = [b"-qn", b"3"];
    let p = parse(&head_spec(), &args).unwrap();
    assert!(p.has(b'q'));
    assert_eq!(p.value(b'n'), Some(&b"3"[..]));
}

#[test]
fn a_value_flag_with_nothing_after_it_is_an_error() {
    let args: [&[u8]; 1] = [b"-n"];
    let err = parse(&head_spec(), &args).unwrap_err();
    assert_eq!(err, b"head: missing value for -n".to_vec());
}

#[test]
fn word_options_take_the_next_argument_whole() {
    let spec = Spec::new(b"find", b"").words(&[b"name", b"type"]);
    let args: [&[u8]; 5] = [b"/etc", b"-name", b"*.cfg", b"-type", b"f"];
    let p = parse(&spec, &args).unwrap();
    assert_eq!(p.word(b"name"), Some(&b"*.cfg"[..]));
    assert_eq!(p.word(b"type"), Some(&b"f"[..]));
    assert_eq!(p.operands, [b"/etc"]);
}

#[test]
fn a_word_option_is_not_treated_as_a_cluster() {
    let spec = Spec::new(b"find", b"");
    let args: [&[u8]; 1] = [b"-name"];
    assert!(parse(&spec, &args).is_err());
}

#[test]
fn parse_usize_rejects_anything_but_digits() {
    assert_eq!(parse_usize(b"120"), Some(120));
    assert_eq!(parse_usize(b""), None);
    assert_eq!(parse_usize(b"1x"), None);
}
