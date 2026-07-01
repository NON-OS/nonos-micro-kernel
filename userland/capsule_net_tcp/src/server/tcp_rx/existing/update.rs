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

use crate::server::tcp_rx::action::RxAction;
use crate::server::tcp_rx::existing::{apply_accept, apply_reap, no_match, step_entry};
use crate::state::{TimerKind, TABLE};
use crate::tcp::{Endpoint4, TcpHeader};

pub fn update(local: Endpoint4, remote: Endpoint4, hdr: TcpHeader, payload: &[u8]) -> RxAction {
    let now = crate::clock::now_ms();
    let mut table = TABLE.lock();
    let mut arm: Option<(u32, u64)> = None;
    let mut accepted: Option<(u32, u32)> = None;
    let action = match table.connection_match_mut(local, remote) {
        Some(e) => step_entry::step_entry(e, &hdr, payload, now, &mut accepted, &mut arm),
        None => {
            let has_listener = table.listener_for_mut(local.port).is_some();
            no_match::no_match(local, remote, &hdr, has_listener)
        }
    };
    if let Some((h, d)) = arm {
        table.timers.arm(h, TimerKind::TimeWait, d);
    }
    drop(table);
    if let Some((parent, child)) = accepted {
        if !apply_accept::apply_accept(parent, child) {
            return RxAction::None;
        }
    }
    if let RxAction::Reap(h) = action {
        apply_reap::apply_reap(h);
        return RxAction::None;
    }
    action
}
