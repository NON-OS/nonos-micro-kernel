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
use nonos_policy_proto::{enum_label, enum_table, Field};

use crate::settings::theme::{VALUE_FG, VALUE_FG_FALSE};

use super::fmt_dec::fmt_dec;
use super::layout::VALUE_LEFT;

pub fn paint_value_enum(fb: &mut PaintBuffer, y: u32, field: Field, value: Option<u8>) {
    fb.text(VALUE_LEFT, y, b"<", VALUE_FG);
    let label: &[u8] = value
        .and_then(|v| enum_label(field, v))
        .unwrap_or(b"...");
    fb.text(VALUE_LEFT + 18, y, label, VALUE_FG);
    fb.text(VALUE_LEFT + 180, y, b">", VALUE_FG);
    if let (Some(v), Some(table)) = (value, enum_table(field)) {
        paint_index(fb, y, v as u32, table.len() as u32);
    }
}

fn paint_index(fb: &mut PaintBuffer, y: u32, idx: u32, total: u32) {
    let mut buf = [0u8; 4];
    let n = fmt_dec(idx + 1, &mut buf);
    fb.text(VALUE_LEFT + 200, y, b"[", VALUE_FG_FALSE);
    fb.text(VALUE_LEFT + 208, y, &buf[..n], VALUE_FG_FALSE);
    fb.text(VALUE_LEFT + 230, y, b"/", VALUE_FG_FALSE);
    let mut t = [0u8; 4];
    let tn = fmt_dec(total, &mut t);
    fb.text(VALUE_LEFT + 238, y, &t[..tn], VALUE_FG_FALSE);
    fb.text(VALUE_LEFT + 260, y, b"]", VALUE_FG_FALSE);
}
