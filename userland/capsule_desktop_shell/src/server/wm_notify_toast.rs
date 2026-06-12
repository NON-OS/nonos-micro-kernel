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

use nonos_libc::mk_time_millis;

use super::wm_notify_label::resolve_label;
use crate::state::{Context, NotifyLevel};

pub fn toast_window_event(ctx: &mut Context, opened: bool, owner_pid: u32) {
    let Some(label) = resolve_label(owner_pid) else {
        return;
    };
    let verb: &[u8] = if opened { b" opened" } else { b" closed" };
    let mut text = [0u8; 48];
    let n = label.len().min(40);
    text[..n].copy_from_slice(&label[..n]);
    text[n..n + verb.len()].copy_from_slice(verb);
    ctx.toasts.push(&text[..n + verb.len()], NotifyLevel::Info, mk_time_millis());
}
