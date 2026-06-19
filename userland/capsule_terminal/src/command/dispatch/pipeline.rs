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

// Run a `a | b | c` pipeline: split the args on `|`, execute the first
// segment with its output captured, then fold each subsequent segment as
// a filter over the accumulated lines. Returns the final lines for the
// caller to display or redirect.
pub(super) fn run_pipeline(state: &mut State, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut segments: Vec<&[&[u8]]> = Vec::new();
    let mut start = 0;
    for i in 0..=args.len() {
        if i == args.len() || args[i] == b"|" {
            if i > start {
                segments.push(&args[start..i]);
            }
            start = i + 1;
        }
    }
    if segments.is_empty() {
        return Vec::new();
    }
    state.scrollback.begin_capture();
    let _ = exec(state, segments[0]);
    let mut lines = state.scrollback.end_capture();
    for seg in &segments[1..] {
        lines = apply(seg, lines);
    }
    lines
}

// Apply a `a | b | c` filter chain to pre-seeded input lines with no
// producer command: used when `< file` supplies the input instead of a
// leading command's captured output.
pub(super) fn run_filters(seed: Vec<Vec<u8>>, args: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut lines = seed;
    let mut start = 0;
    for i in 0..=args.len() {
        if i == args.len() || args[i] == b"|" {
            if i > start {
                lines = apply(&args[start..i], lines);
            }
            start = i + 1;
        }
    }
    lines
}
