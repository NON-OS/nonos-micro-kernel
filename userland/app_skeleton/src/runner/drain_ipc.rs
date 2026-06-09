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

use nonos_libc::mk_ipc_recv_from;

use crate::app::{App, EventOutcome};

use super::{click_focus, control::handle_control, decorations, dispatch::parse_delivery};

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;

pub(super) struct DrainResult {
    pub repaint: bool,
    pub close: bool,
    pub minimize: bool,
}

pub(super) fn drain<A: App>(
    app: &mut A,
    rx: &mut [u8],
    width: u32,
    wm_port: u32,
    window_id: u32,
    request_id: &mut u32,
) -> DrainResult {
    let mut repaint = false;
    loop {
        let mut sender = 0u32;
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), RECV_NOWAIT, &mut sender);
        if n <= 0 {
            return DrainResult { repaint, close: false, minimize: false };
        }
        if handle_control(&rx[..n as usize], sender, wm_port, window_id, request_id) {
            continue;
        }
        let Some(event) = parse_delivery(&rx[..n as usize]) else { continue };
        let event = decorations::normalize(event);
        click_focus::handle(event, wm_port, window_id, request_id);
        match decorations::handle(width, event) {
            Some(EventOutcome::Close) => return DrainResult { repaint, close: true, minimize: false },
            Some(EventOutcome::Minimize) => {
                return DrainResult { repaint, close: false, minimize: true }
            }
            _ => {}
        }
        match app.on_event(event) {
            EventOutcome::Idle | EventOutcome::Minimize => {}
            EventOutcome::Repaint => repaint = true,
            EventOutcome::Close => return DrainResult { repaint, close: true, minimize: false },
        }
    }
}
