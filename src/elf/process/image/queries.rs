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

use super::state::{ProcessImage, ProcessState};

impl ProcessImage {
    pub fn has_interpreter(&self) -> bool {
        self.interpreter.is_some()
    }

    pub fn has_tls(&self) -> bool {
        self.tls.is_some()
    }

    pub fn is_ready(&self) -> bool {
        self.state == ProcessState::Ready
    }

    pub fn is_running(&self) -> bool {
        self.state == ProcessState::Running
    }

    pub fn is_terminated(&self) -> bool {
        self.state == ProcessState::Terminated
    }
}
