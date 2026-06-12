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

pub const HISTORY: usize = 30;

#[derive(Clone, Copy)]
pub struct CpuSample {
    pub percent: [u8; HISTORY],
    pub head: usize,
}

impl CpuSample {
    pub const ZERO: CpuSample = CpuSample { percent: [0; HISTORY], head: 0 };
}

#[derive(Clone, Copy)]
pub struct Row {
    pub label: &'static [u8],
    pub service: &'static [u8],
    pub pid: u32,
    pub online: bool,
    pub cpu: CpuSample,
}

const EMPTY_ROW: Row =
    Row { label: b"", service: b"", pid: 0, online: false, cpu: CpuSample::ZERO };
const KNOWN: [Row; 8] = [
    Row { label: b"terminal", service: b"app.terminal", ..EMPTY_ROW },
    Row { label: b"file_manager", service: b"app.file_manager", ..EMPTY_ROW },
    Row { label: b"text_editor", service: b"app.text_editor", ..EMPTY_ROW },
    Row { label: b"settings", service: b"app.settings", ..EMPTY_ROW },
    Row { label: b"process_manager", service: b"app.process_manager", ..EMPTY_ROW },
    Row { label: b"about", service: b"app.about", ..EMPTY_ROW },
    Row { label: b"calculator", service: b"app.calculator", ..EMPTY_ROW },
    Row { label: b"desktop_shell", service: b"desktop_shell", ..EMPTY_ROW },
];

pub struct State {
    pub rows: [Row; 8],
    pub refreshes: u32,
    pub status: &'static [u8],
    pub last_total_ticks: u64,
    pub last_ticks: [u64; 8],
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            rows: [EMPTY_ROW; 8],
            refreshes: 0,
            status: b"lookup pending",
            last_total_ticks: 0,
            last_ticks: [0; 8],
        };
        state.refresh();
        state
    }

    pub fn refresh(&mut self) {
        self.refreshes = self.refreshes.wrapping_add(1);
        for (row, known) in self.rows.iter_mut().zip(KNOWN.iter()) {
            row.label = known.label;
            row.service = known.service;
            row.pid = 0;
            row.online = false;
            if let Some(peer) = lookup_service(row.service) {
                row.pid = peer.pid;
                row.online = true;
            }
        }
        self.status = b"PID from service lookup, caps unavailable";
    }
}
