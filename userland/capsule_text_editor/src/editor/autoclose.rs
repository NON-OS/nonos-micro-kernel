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

//! Bracket and quote assistance for printable input: wrap a selection in the
//! typed pair, auto-insert the closer after an opener, and step over a closer
//! that is already there. All the punctuation involved is ASCII, so a byte
//! stands in for the character throughout.

use alloc::vec::Vec;

use super::state::State;

pub(super) enum Auto {
    // The buffer changed (a pair was inserted or a selection wrapped).
    Inserted,
    // Only the caret moved, stepping over an existing closer.
    Skipped,
}

// The closing byte for an opening bracket, if `ch` opens one.
fn closer_for(ch: u8) -> Option<u8> {
    match ch {
        b'(' => Some(b')'),
        b'[' => Some(b']'),
        b'{' => Some(b'}'),
        _ => None,
    }
}

fn is_closer(ch: u8) -> bool {
    matches!(ch, b')' | b']' | b'}')
}

fn is_quote(ch: u8) -> bool {
    matches!(ch, b'"' | b'\'' | b'`')
}

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80
}

// The byte to close a wrap of a selection with `ch`: the matching bracket, or
// the same quote.
fn wrap_close(ch: u8) -> Option<u8> {
    closer_for(ch).or(if is_quote(ch) { Some(ch) } else { None })
}

pub(super) fn autoclose(state: &mut State, ch: char) -> Option<Auto> {
    if !ch.is_ascii() {
        return None;
    }
    let ch = ch as u8;
    let next = state.buf[..state.len].get(state.caret).copied();
    let prev = if state.caret > 0 { state.buf.get(state.caret - 1).copied() } else { None };

    // Wrap a selection in the typed bracket or quote.
    if let Some((s, e)) = state.sel_range() {
        if let Some(close) = wrap_close(ch) {
            let mut ins = Vec::with_capacity(e - s + 2);
            ins.push(ch);
            ins.extend_from_slice(&state.buf[s..e]);
            ins.push(close);
            if state.apply_edit(s, e - s, &ins) {
                state.sel_anchor = Some(s + 1);
                state.caret = e + 1;
                return Some(Auto::Inserted);
            }
        }
        return None;
    }

    // Step over a closer or quote that already sits under the caret.
    if (is_closer(ch) || is_quote(ch)) && next == Some(ch) {
        state.caret += 1;
        return Some(Auto::Skipped);
    }

    // Auto-insert the matching bracket, leaving the caret between the pair.
    if let Some(close) = closer_for(ch) {
        if next.map(is_word).unwrap_or(false) {
            return None; // typing "(" right before a word: don't get in the way
        }
        if state.apply_edit(state.caret, 0, &[ch, close]) {
            state.caret -= 1;
            return Some(Auto::Inserted);
        }
        return None;
    }

    // Auto-close a quote, but not when it is more likely an apostrophe (after a
    // word) or the open side of an already-typed run (before a word).
    if is_quote(ch) {
        let after_word = prev.map(is_word).unwrap_or(false);
        let before_word = next.map(is_word).unwrap_or(false);
        if after_word || before_word {
            return None;
        }
        if state.apply_edit(state.caret, 0, &[ch, ch]) {
            state.caret -= 1;
            return Some(Auto::Inserted);
        }
    }

    None
}
