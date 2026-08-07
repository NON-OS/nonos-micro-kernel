// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for reverse history search. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/search_host.rs -o /tmp/search_host && /tmp/search_host
//
// The substring rule is the shipping one, pulled in by path.

const COLS: usize = 256;
const HISTORY_DEPTH: usize = 64;

mod types {
    use super::{COLS, HISTORY_DEPTH};
    pub struct History {
        pub(super) entries: [[u8; COLS]; HISTORY_DEPTH],
        pub(super) lengths: [usize; HISTORY_DEPTH],
        pub(super) count: usize,
        pub(super) cursor: Option<usize>,
    }
    impl History {
        pub fn new() -> Self {
            Self {
                entries: [[0; COLS]; HISTORY_DEPTH],
                lengths: [0; HISTORY_DEPTH],
                count: 0,
                cursor: None,
            }
        }
        pub fn push(&mut self, line: &[u8]) {
            let n = line.len().min(COLS);
            self.entries[self.count][..n].copy_from_slice(&line[..n]);
            self.lengths[self.count] = n;
            self.count += 1;
            let _ = self.cursor;
        }
    }
}

#[path = "../src/term/history/search_back.rs"]
mod search_back;

use types::History;

fn history(lines: &[&str]) -> History {
    let mut h = History::new();
    for line in lines {
        h.push(line.as_bytes());
    }
    h
}

/// The point of a search rather than a prefix recall: what a reader
/// remembers is usually a word from the middle of a command.
#[test]
fn a_word_from_the_middle_finds_the_line() {
    let h = history(&["ls /tmp", "curl https://example.com", "echo hi"]);
    let (_, line) = h.search_back(b"example", 3).expect("the middle word must match");
    assert_eq!(line, b"curl https://example.com");
}

/// Newest first, because the command a reader wants again is usually the one
/// they ran most recently.
#[test]
fn the_newest_match_comes_first() {
    let h = history(&["git status", "git log", "ls"]);
    let (at, line) = h.search_back(b"git", 3).expect("a match must be found");
    assert_eq!(line, b"git log");
    assert_eq!(at, 1);
}

/// Pressing the key again has to move, which is what stepping past the match
/// on screen means.
#[test]
fn stepping_back_finds_the_older_match() {
    let h = history(&["git status", "git log", "ls"]);
    let (at, _) = h.search_back(b"git", 3).expect("first match");
    let (_, older) = h.search_back(b"git", at).expect("an older match must be found");
    assert_eq!(older, b"git status");
}

/// An empty search shows the newest line, which is what pressing up once
/// would give, rather than nothing.
#[test]
fn an_empty_search_shows_the_newest_line() {
    let h = history(&["one", "two"]);
    let (_, line) = h.search_back(b"", 2).expect("an empty needle matches");
    assert_eq!(line, b"two");
}

#[test]
fn a_needle_that_matches_nothing_finds_nothing() {
    let h = history(&["ls", "pwd"]);
    assert!(h.search_back(b"zzz", 2).is_none());
}

/// A needle longer than any line cannot match, and must not read past the
/// end of one looking.
#[test]
fn a_needle_longer_than_the_line_is_refused() {
    let h = history(&["ls"]);
    assert!(h.search_back(b"lslslslsls", 1).is_none());
}

#[test]
fn searching_an_empty_history_finds_nothing() {
    let h = history(&[]);
    assert!(h.search_back(b"x", 0).is_none());
}
