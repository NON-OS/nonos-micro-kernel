// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
//
// Host-side harness for the `ls` argument parser. Compiled with the host
// toolchain so clustering, `--`, repeats and unknown options are exercised
// without QEMU:
//   rustc --edition 2021 --test tests/lsflags_host.rs -o /tmp/lsflags_host && /tmp/lsflags_host
//
// The module is pulled in by path so this cannot pass against a copy that has
// drifted from what the capsule runs.

extern crate alloc;

#[path = "../src/command/builtin/fs/ls_flags.rs"]
mod ls_flags;

use ls_flags::{parse, LsFlags};

fn run<'a>(args: &[&'a [u8]]) -> (LsFlags, Vec<&'a [u8]>) {
    parse(args).expect("expected the arguments to parse")
}

#[test]
fn bare_ls_sets_nothing_and_takes_no_operand() {
    let (flags, operands) = run(&[b"ls"]);
    assert_eq!(flags, LsFlags::default());
    assert!(operands.is_empty());
}

/// The reason this exists: `ls -lah` used to be treated as a path.
#[test]
fn clustered_flags_all_apply() {
    let (flags, operands) = run(&[b"ls", b"-lah"]);
    assert!(flags.long && flags.all && flags.human);
    assert!(!flags.recurse && !flags.by_time && !flags.by_size);
    assert!(operands.is_empty());
}

#[test]
fn separate_flags_match_the_clustered_form() {
    assert_eq!(run(&[b"ls", b"-l", b"-a", b"-h"]).0, run(&[b"ls", b"-lah"]).0);
    assert_eq!(run(&[b"ls", b"-1RtS"]).0, run(&[b"ls", b"-1", b"-R", b"-t", b"-S"]).0);
}

#[test]
fn repeated_flags_are_idempotent() {
    assert_eq!(run(&[b"ls", b"-l", b"-ll", b"-l"]).0, run(&[b"ls", b"-l"]).0);
}

#[test]
fn double_dash_ends_the_flag_run() {
    let (flags, operands) = run(&[b"ls", b"-l", b"--", b"-a", b"--", b"-lah"]);
    assert!(flags.long && !flags.all);
    assert_eq!(operands, vec![&b"-a"[..], &b"--"[..], &b"-lah"[..]]);
}

#[test]
fn every_non_flag_word_is_an_operand() {
    let (flags, operands) = run(&[b"ls", b"/a", b"-l", b"/b", b"/c"]);
    assert!(flags.long);
    assert_eq!(operands, vec![&b"/a"[..], &b"/b"[..], &b"/c"[..]]);
}

/// A lone dash is a path operand, not an empty flag cluster.
#[test]
fn lone_dash_is_an_operand() {
    let (flags, operands) = run(&[b"ls", b"-"]);
    assert_eq!(flags, LsFlags::default());
    assert_eq!(operands, vec![&b"-"[..]]);
}

#[test]
fn unknown_flags_are_rejected_not_ignored() {
    for (args, bad) in [
        (vec![&b"ls"[..], &b"-z"[..]], b'z'),
        (vec![&b"ls"[..], &b"-laz"[..]], b'z'),
        (vec![&b"ls"[..], &b"-l"[..], &b"-Q"[..]], b'Q'),
    ] {
        assert_eq!(parse(&args), Err(bad), "expected {} to be rejected", bad as char);
    }
}

#[test]
fn an_unknown_flag_is_never_taken_as_a_path() {
    assert!(parse(&[&b"ls"[..], &b"-lah"[..]]).is_ok());
    assert!(parse(&[&b"ls"[..], &b"--"[..], &b"-lah"[..]]).is_ok());
    assert!(parse(&[&b"ls"[..], &b"-lax"[..]]).is_err());
}

#[test]
fn needs_meta_tracks_the_flags_that_require_a_stat() {
    assert!(!run(&[b"ls", b"-ah1"]).0.needs_meta());
    for arg in [&b"-l"[..], &b"-t"[..], &b"-S"[..]] {
        assert!(run(&[b"ls", arg]).0.needs_meta(), "{:?}", arg);
    }
}
