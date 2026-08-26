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

use super::super::card;
use super::super::chrome::Rect;
use super::super::metrics::CARD_GAP;
use super::sec_alerts;

// The same tally and the same findings the old panel drew, on the shared
// primitives: the posture as stat cards, then the monitor's findings as rows.
// Nothing here is computed differently, only painted differently.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    posture(state, fb, r);
    sec_alerts::paint(state, fb, r);
}

// One card per sensitive class the posture counts. DMA keeps a card of its own
// because it is the one authority that bypasses the page tables outright.
fn posture(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let p = &state.monitor.posture;
    let cells: [(IconId, &[u8], &[u8], u32); 6] = [
        (IconId::Processes, b"PROCESSES", b"live", p.total),
        (IconId::SettingsSecurity, b"ADMIN", b"blanket", p.admin),
        (IconId::SettingsDeveloper, b"RAW HW", b"device", p.raw_hw),
        (IconId::SettingsStorage, b"DMA", b"bypass", p.dma),
        (IconId::SettingsGeneral, b"SPAWN", b"reach", p.spawn),
        (IconId::SettingsPrivacy, b"DEBUG", b"inspect", p.debug),
    ];
    let n = cells.len() as u32;
    let w = r.w.saturating_sub(CARD_GAP * (n - 1)) / n;
    for (i, (icon, caption, sub, value)) in cells.iter().enumerate() {
        let mut buf = [0u8; 12];
        let len = u32_decimal(*value, &mut buf);
        let x = r.x + i as u32 * (w + CARD_GAP);
        card::paint(fb, x, r.y, w, *icon, caption, &buf[..len], b"", sub);
    }
}
