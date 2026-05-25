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

use crate::elf::tls::TlsInfo;

use super::state::{ProcessImage, ProcessState};

impl ProcessImage {
    pub fn set_tls(&mut self, tls: TlsInfo) {
        self.tls = Some(tls);
    }

    pub fn set_ready(&mut self) {
        self.state = ProcessState::Ready;
    }

    pub fn set_running(&mut self) {
        self.state = ProcessState::Running;
    }

    pub fn set_blocked(&mut self) {
        self.state = ProcessState::Blocked;
    }

    pub fn set_terminated(&mut self) {
        self.state = ProcessState::Terminated;
    }
}
