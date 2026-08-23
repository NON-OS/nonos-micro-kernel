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

//! The window has two panels over one live data set: the process table and a
//! security view. Shared drawing helpers and layout live here; each panel is
//! its own file.

mod security;
mod table;

use nonos_app_skeleton::PaintBuffer;

use super::security::Level;
use super::state::{Screen, State};
use super::theme::{AMBER, BACKGROUND, DANGER, OK};

pub(super) use super::layout::HEADER_H;

pub(super) const PAD: u32 = 18;
pub(super) const HEADER_TEXT_Y: u32 = 13;
pub(super) const FIRST_ROW_Y: u32 = 50;
pub(super) const ROW_H: u32 = 23;
pub(super) const ROW_TEXT_DY: u32 = 4;
pub(super) const BODY_PX: f32 = 15.0;
pub(super) const SMALL_PX: f32 = 14.0;

// Draw a byte label as crisp monospace TrueType.
pub(super) fn t(fb: &mut PaintBuffer, x: u32, y: u32, bytes: &[u8], color: u32, px: f32) {
    let s = core::str::from_utf8(bytes).unwrap_or("");
    let _ = fb.text_ttf_mono(x as i32, y as i32, s, color, px);
}

// The always-visible security badge at the top right: green when clear, else
// the worst finding's colour with a count. Same in both panels so the security
// posture is never out of sight.
pub(super) fn badge(fb: &mut PaintBuffer, state: &State) {
    let worst = state.alerts.iter().map(|a| a.level).max();
    let (color, label): (u32, &[u8]) = match worst {
        None => (OK, b"SECURE"),
        Some(Level::Info) => (OK, b"REVIEW"),
        Some(Level::Warn) => (AMBER, b"WARNING"),
        Some(Level::Critical) => (DANGER, b"CRITICAL"),
    };
    let x = fb.width.saturating_sub(150);
    fb.fill_rect(x, 9, 6, 18, color);
    t(fb, x + 14, HEADER_TEXT_Y, label, color, BODY_PX);
    if !state.alerts.is_empty() {
        let mut buf = [0u8; 8];
        let n = super::format::u32_decimal(state.alerts.len() as u32, &mut buf);
        t(fb, x + 108, HEADER_TEXT_Y, &buf[..n], color, BODY_PX);
    }
}

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    match state.screen {
        Screen::Overview | Screen::Processes => table::paint(state, fb),
        Screen::Cpu | Screen::Memory | Screen::Authority | Screen::Security => {
            security::paint(state, fb)
        }
    }
}
