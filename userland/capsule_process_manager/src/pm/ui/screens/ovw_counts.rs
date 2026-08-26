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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;

use crate::pm::format::u32_decimal;
use crate::pm::state::State;
use crate::pm::theme::{AMBER, OK};

use super::super::card;
use super::super::risk_strip::CLASSES;
use super::ovw_cards::{meter, sub_n};

// A privileged process is one holding any bit the risk strip draws, so this
// count and every strip in the table can never disagree about who is dangerous.
fn privileged(caps: u64) -> bool {
    CLASSES.iter().any(|(mask, _)| caps & mask != 0)
}

// Running is the only figure the card claims, because it is the one state the
// kernel reports outright; the rest are parked and the table spells them out.
pub(super) fn processes(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let total = state.rows.len() as u64;
    let running = state.rows.iter().filter(|row| row.state == 2).count() as u64;
    let mut buf = [0u8; 24];
    let n = u32_decimal(total as u32, &mut buf);
    let mut sub = [0u8; 24];
    let s = sub_n(&mut sub, b"", running as u32, b" running");
    let icon = IconId::Processes;
    let band = card::paint(fb, x, y, w, icon, b"PROCESSES", &buf[..n], b"", &sub[..s]);
    meter(fb, (x, y, w), band, running, total, OK);
}

pub(super) fn authority(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let total = state.rows.len() as u64;
    let held = state.rows.iter().filter(|row| privileged(row.caps)).count() as u64;
    let mut buf = [0u8; 24];
    let n = u32_decimal(held as u32, &mut buf);
    let mut sub = [0u8; 24];
    let s = sub_n(&mut sub, b"of ", total as u32, b" processes");
    let icon = IconId::SettingsSecurity;
    let band = card::paint(fb, x, y, w, icon, b"PRIVILEGED", &buf[..n], b"", &sub[..s]);
    meter(fb, (x, y, w), band, held, total, AMBER);
}
