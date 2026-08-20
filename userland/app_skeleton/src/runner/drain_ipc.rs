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

use super::control::{handle_control, ControlOutcome};
use super::drag::{self, DragState, PointerAction};
use super::{click_focus, decorations, dispatch::parse_delivery};

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;

#[derive(Default)]
pub(super) struct DrainResult {
    pub repaint: bool,
    pub close: bool,
    pub minimize: bool,
    pub maximize: bool,
    pub restore: bool,
    pub move_to: Option<(u32, u32)>,
    pub resize_to: Option<(u32, u32)>,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn drain<A: App>(
    app: &mut A,
    drag_state: &mut DragState,
    rx: &mut [u8],
    width: u32,
    height: u32,
    win_x: u32,
    win_y: u32,
    wm_port: u32,
    window_id: u32,
    request_id: &mut u32,
    maximized: bool,
) -> DrainResult {
    let mut repaint = false;
    let mut restore = false;
    let mut move_to = None;
    let mut resize_to = None;
    loop {
        let mut sender = 0u32;
        let n =
            mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), RECV_NOWAIT, &mut sender);
        if n <= 0 {
            return DrainResult { repaint, restore, move_to, resize_to, ..Default::default() };
        }
        match handle_control(&rx[..n as usize], sender, wm_port, window_id, request_id) {
            ControlOutcome::FocusSelf => {
                restore = true;
                continue;
            }
            ControlOutcome::Handled => continue,
            ControlOutcome::NotControl => {}
        }
        let Some(event) = parse_delivery(&rx[..n as usize]) else { continue };
        let event = decorations::normalize(event);
        click_focus::handle(event, wm_port, window_id, request_id);
        match decorations::handle(width, height, maximized, event) {
            Some(EventOutcome::Close) => {
                return DrainResult {
                    repaint,
                    restore,
                    move_to,
                    resize_to,
                    close: true,
                    ..Default::default()
                };
            }
            Some(EventOutcome::Minimize) => {
                return DrainResult {
                    repaint,
                    restore,
                    move_to,
                    resize_to,
                    minimize: true,
                    ..Default::default()
                };
            }
            Some(EventOutcome::Maximize) => {
                return DrainResult {
                    repaint,
                    restore,
                    move_to,
                    resize_to,
                    maximize: true,
                    ..Default::default()
                };
            }
            _ => {}
        }
        match drag::handle(drag_state, width, height, win_x, win_y, maximized, &event) {
            PointerAction::MoveTo(mx, my) => {
                move_to = Some((mx, my));
                continue;
            }
            PointerAction::ResizeTo(rw, rh) => {
                resize_to = Some((rw, rh));
                continue;
            }
            PointerAction::HoverChanged => {
                repaint = true;
                continue;
            }
            PointerAction::None => {}
        }
        match app.on_event(decorations::to_content(width, height, maximized, event)) {
            EventOutcome::Idle | EventOutcome::Minimize | EventOutcome::Maximize => {}
            EventOutcome::Repaint => repaint = true,
            EventOutcome::Close => {
                return DrainResult {
                    repaint,
                    restore,
                    move_to,
                    resize_to,
                    close: true,
                    ..Default::default()
                };
            }
        }
    }
}
