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

//! Whether this wallet is only for this session, said on the card.
//!
//! A wallet the machine cannot keep still works: it signs, it holds a
//! balance, it looks like any other wallet, and it is gone at the next boot.
//! That difference is invisible until it costs someone their funds, so it is
//! shown where the balance is rather than in a status line that scrolls away.
//!
//! The wording is about the machine, not the wallet. "Session only" is the
//! honest description of a machine with no TPM, and it points at the phrase
//! rather than implying something is broken.

use nonos_app_skeleton::PaintBuffer;

use super::scale;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{AMBER, AMBER_INK, GREEN, GREEN_INK};

const KEPT: &[u8] = b"KEPT";
const SESSION: &[u8] = b"SESSION ONLY";

/// Drawn to the left of the ACTIVE badge, so the two read as one row and the
/// stronger claim is the one further from the edge.
pub fn badge(state: &State, fb: &mut PaintBuffer, right_x: u32, y: u32) -> u32 {
    let (text, bg, ink) = if state.vault_saved {
        (KEPT, GREEN(), GREEN_INK())
    } else {
        (SESSION, AMBER(), AMBER_INK())
    };
    let s = core::str::from_utf8(text).unwrap_or("");
    let w = fb.measure_ttf(s, scale::BODY).max(0) as u32 + 18;
    ui::badge(fb, right_x.saturating_sub(w), y, text, bg, ink);
    w
}
