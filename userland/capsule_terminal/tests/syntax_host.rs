// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for input line classification. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/syntax_host.rs -o /tmp/syntax_host && /tmp/syntax_host
//
// The theme is stubbed because only the classification is under test; the
// module is otherwise the shipping one, pulled in by path.

mod term {
    pub mod theme {
        pub mod types {
            #[derive(Clone, Copy)]
            pub struct Theme {
                pub bg: u32,
                pub fg: u32,
                pub accent: u32,
                pub path: u32,
                pub dim: u32,
                pub ok: u32,
                pub warn: u32,
                pub err: u32,
                pub run: u32,
                pub chrome_edge: u32,
            }
        }
    }
}
use term as crate_term;

#[path = "../src/paint/syntax/mod.rs"]
mod syntax;

use syntax::{classify, Part};

fn parts_of(line: &str) -> Vec<Part> {
    let mut out = vec![Part::Plain; line.len()];
    classify(line.as_bytes(), &mut out);
    out
}

/// The command is the one word that decides what the rest of the line means,
/// so it is the one that carries the accent.
#[test]
fn the_first_word_is_the_command() {
    let p = parts_of("ls -la /etc");
    assert_eq!(p[0], Part::Command);
    assert_eq!(p[1], Part::Command);
    assert_eq!(p[2], Part::Plain, "the space is not part of it");
}

#[test]
fn a_dashed_word_is_a_flag() {
    let p = parts_of("ls -la");
    assert_eq!(p[3], Part::Flag);
    assert_eq!(p[5], Part::Flag);
}

/// A path is coloured from its first character, not from the slash part way
/// along it.
#[test]
fn a_word_with_a_separator_is_a_path_from_its_start() {
    let p = parts_of("cat etc/hosts");
    assert_eq!(p[4], Part::Path, "starts at the e, not the slash");
    assert_eq!(p[7], Part::Path);
}

#[test]
fn a_plain_argument_stays_plain() {
    let p = parts_of("echo hello");
    assert_eq!(p[5], Part::Plain);
}

/// After a pipe the line starts again, so the next word is a command rather
/// than an argument of the one before.
#[test]
fn a_word_after_an_operator_is_a_command_again() {
    let line = "cat f | grep x";
    let p = parts_of(line);
    let pipe = line.find('|').unwrap();
    assert_eq!(p[pipe], Part::Operator);
    let grep = line.find("grep").unwrap();
    assert_eq!(p[grep], Part::Command);
}

#[test]
fn quoted_text_is_one_part_including_its_spaces() {
    let line = "echo \"a b\"";
    let p = parts_of(line);
    let open = line.find('"').unwrap();
    for i in open..line.len() {
        assert_eq!(p[i], Part::Quoted, "byte {i}");
    }
}

/// An unclosed quote runs to the end rather than falling back mid line, which
/// is what shows the reader the quote is still open.
#[test]
fn an_unclosed_quote_runs_to_the_end() {
    let line = "echo \"still open";
    let p = parts_of(line);
    assert_eq!(p[line.len() - 1], Part::Quoted);
}

/// The caller draws by byte, so every byte has to be classified even when the
/// buffer given is shorter than the line.
#[test]
fn a_short_buffer_is_not_written_past() {
    let line = "ls -la /etc";
    let mut out = vec![Part::Plain; 4];
    classify(line.as_bytes(), &mut out);
    assert_eq!(out.len(), 4);
    assert_eq!(out[0], Part::Command);
}
