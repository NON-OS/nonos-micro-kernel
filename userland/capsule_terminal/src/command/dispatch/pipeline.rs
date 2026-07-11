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

use super::exec::exec;
use super::filter::apply;
use crate::term::state::State;

const FILTERS: [&[u8]; 8] = [b"grep", b"sort", b"uniq", b"cut", b"nl", b"wc", b"head", b"tail"];

// Run a `a | b | c` pipeline: split the args on `|`, then fold each stage
// in order over an accumulating buffer. A stage whose command name is a
// known filter runs as a filter over the buffer; any other stage runs as
// a real command through `exec`, capturing its output as the new buffer.
// v1: real-command stages ignore the incoming buffer (no stdin plumbing
// into non-filter commands) — only filters read upstream output.
pub(super) fn run_pipeline(state: &mut State, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let segments = split_stages(args);
    if segments.is_empty() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    for seg in &segments {
        lines = run_stage(state, seg, lines);
    }
    lines
}

pub(crate) fn run_stage(state: &mut State, seg: &[&[u8]], buffer: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    if FILTERS.contains(&seg.first().copied().unwrap_or(b"")) {
        apply(seg, buffer)
    } else {
        state.scrollback.begin_capture();
        let _ = exec(state, seg);
        state.scrollback.end_capture()
    }
}

pub(crate) fn split_stages<'a>(args: &'a [&'a [u8]]) -> Vec<&'a [&'a [u8]]> {
    let mut segments = Vec::new();
    let mut start = 0;
    for i in 0..=args.len() {
        if i == args.len() || args[i] == b"|" {
            if i > start {
                segments.push(&args[start..i]);
            }
            start = i + 1;
        }
    }
    segments
}

// Apply a `a | b | c` filter chain to pre-seeded input lines with no
// producer command: used when `< file` supplies the input instead of a
// leading command's captured output.
pub(super) fn run_filters(seed: Vec<Vec<u8>>, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut lines = seed;
    for seg in &split_stages(args) {
        lines = apply(seg, lines);
    }
    lines
}
