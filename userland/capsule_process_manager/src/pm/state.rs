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

use nonos_app_skeleton::discover::lookup_service;

#[derive(Clone, Copy)]
pub struct Row {
    pub label: &'static [u8],
    pub service: &'static [u8],
    pub pid: u32,
    pub online: bool,
}

const EMPTY_ROW: Row = Row { label: b"", service: b"", pid: 0, online: false };
const KNOWN: [Row; 8] = [
    Row { label: b"terminal", service: b"app.terminal", pid: 0, online: false },
    Row { label: b"file_manager", service: b"app.file_manager", pid: 0, online: false },
    Row { label: b"text_editor", service: b"app.text_editor", pid: 0, online: false },
    Row { label: b"settings", service: b"app.settings", pid: 0, online: false },
    Row { label: b"process_manager", service: b"app.process_manager", pid: 0, online: false },
    Row { label: b"about", service: b"app.about", pid: 0, online: false },
    Row { label: b"calculator", service: b"app.calculator", pid: 0, online: false },
    Row { label: b"desktop_shell", service: b"desktop_shell", pid: 0, online: false },
];

pub struct State {
    pub rows: [Row; 8],
    pub refreshes: u32,
    pub status: &'static [u8],
}

impl State {
    pub fn new() -> Self {
        let mut state = State { rows: [EMPTY_ROW; 8], refreshes: 0, status: b"lookup pending" };
        state.refresh();
        state
    }

    pub fn refresh(&mut self) {
        self.refreshes = self.refreshes.wrapping_add(1);
        self.rows = KNOWN;
        for row in self.rows.iter_mut() {
            if let Some(peer) = lookup_service(row.service) {
                row.pid = peer.pid;
                row.online = true;
            }
        }
        self.status = b"PID from service lookup, caps unavailable";
    }
}
