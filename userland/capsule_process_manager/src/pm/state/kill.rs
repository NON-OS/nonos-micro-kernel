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

use nonos_libc::mk_kill;

use super::super::critical::is_critical;
use super::{State, SIGKILL};

impl State {
    // Cancel any armed kill. Called whenever the user navigates or re-sorts, so
    // a pending confirmation never carries over to a different selection.
    pub(super) fn disarm(&mut self) {
        self.pending_pid = 0;
        self.pending_sig = 0;
    }

    // Terminate the selected process with `sig`, in two steps. The first press
    // arms the exact (pid, signal); the same press again confirms and sends it.
    // Core system processes are refused outright: ending the compositor, window
    // manager, input router or a core service would strand the whole session, so
    // the monitor never lets a keypress do it. Ordinary apps stay killable. The
    // kernel still enforces the ProcessControl authority on top of this.
    pub fn kill_selected(&mut self, sig: u64) {
        let Some(pid) = (self.selected_pid != 0).then_some(self.selected_pid) else {
            self.notice = b"select a process first";
            return;
        };
        if self.rows.iter().find(|r| r.pid == pid).is_some_and(|r| is_critical(r.name())) {
            self.notice = b"protected: a core system process cannot be ended here";
            self.disarm();
            return;
        }
        if self.pending_pid == pid && self.pending_sig == sig {
            let rc = mk_kill(pid as u64, sig);
            self.notice = if rc >= 0 {
                if sig == SIGKILL {
                    b"force-killed"
                } else {
                    b"terminated"
                }
            } else {
                b"denied by the kernel"
            };
            self.disarm();
            self.refresh();
            return;
        }
        self.pending_pid = pid;
        self.pending_sig = sig;
        self.notice = b"press the same key again to confirm";
    }
}
