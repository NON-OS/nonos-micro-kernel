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

use crate::about::data::trust;
use crate::about::theme::{FOREGROUND, MUTED};

use super::super::card::{self, titled};
use super::super::kv::{kv, ROW_H};
use super::super::metrics::{BODY_PX, CARD_PAD, CHAIN_H, PAIR_LINE};
use super::super::text::line;
use super::trust_hops::nodes;
use super::{chain, prose};

const CHAIN_GAP: u32 = 14;
const EVIDENCE_GAP: u32 = 12;

pub fn height(inner: u32) -> u32 {
    let evidence = PAIR_LINE + prose::height(trust::STATUS, inner);
    card::OVERHEAD + CHAIN_H + CHAIN_GAP + ROW_H * 3 + EVIDENCE_GAP + evidence
}

pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    let inner = card::inner(w);
    let top = titled(fb, 0, y, w, height(inner), b"Signing chain");
    let mut hops: [&'static [u8]; 4] = [b""; 4];
    let count = nodes(&mut hops);
    chain::paint(fb, CARD_PAD, top, &hops[..count]);
    let rows: [(&[u8], &[u8]); 3] = [
        (b"Scheme", trust::HYBRID_SCHEME),
        (b"Manifest", trust::MANIFEST_FORMAT),
        (b"Certificate", trust::CERT_FORMAT),
    ];
    let first = top + (CHAIN_H + CHAIN_GAP) as i32;
    for (i, (label, value)) in rows.into_iter().enumerate() {
        kv(fb, CARD_PAD, first + (i as u32 * ROW_H) as i32, inner, label, value, false);
    }
    let ev = first + (ROW_H * 3 + EVIDENCE_GAP) as i32;
    line(fb, CARD_PAD, ev, b"Evidence", MUTED, BODY_PX);
    prose::paint(fb, CARD_PAD, ev + PAIR_LINE as i32, inner, trust::STATUS, FOREGROUND);
}
