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

use crate::animation;
use crate::component_dispatch;
use crate::protocol::{
    E_BAD_OP, STATUS_OK, THEME_PAYLOAD_LEN, TOOLKIT_OP_ANIMATION_TICK,
    TOOLKIT_OP_COMPONENT_RENDER, TOOLKIT_OP_HEALTHCHECK, TOOLKIT_OP_THEME_APPLY, TOOLKIT_OP_THEME_GET,
};
use crate::theme;

pub fn dispatch(op: u16, payload: &[u8], reply: &mut [u8]) -> (u16, usize) {
    match op {
        TOOLKIT_OP_HEALTHCHECK => (STATUS_OK, 0),
        TOOLKIT_OP_THEME_APPLY => (theme::apply(payload), 0),
        TOOLKIT_OP_THEME_GET => theme_get(reply),
        TOOLKIT_OP_ANIMATION_TICK => animation::tick(payload, reply),
        TOOLKIT_OP_COMPONENT_RENDER => (component_dispatch::render(payload), 0),
        _ => (E_BAD_OP, 0),
    }
}

fn theme_get(reply: &mut [u8]) -> (u16, usize) {
    if reply.len() < THEME_PAYLOAD_LEN {
        return (E_BAD_OP, 0);
    }
    let snap = theme::snapshot();
    reply[0..4].copy_from_slice(&snap.background_argb.to_le_bytes());
    reply[4..8].copy_from_slice(&snap.surface_argb.to_le_bytes());
    reply[8..12].copy_from_slice(&snap.accent_argb.to_le_bytes());
    reply[12..16].copy_from_slice(&snap.text_argb.to_le_bytes());
    reply[16..20].copy_from_slice(&snap.border_argb.to_le_bytes());
    reply[20..24].copy_from_slice(&snap.revision.to_le_bytes());
    (STATUS_OK, THEME_PAYLOAD_LEN)
}
