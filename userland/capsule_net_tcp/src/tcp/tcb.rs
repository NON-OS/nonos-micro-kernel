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

use super::state::State;

#[derive(Clone, Copy)]
pub struct Endpoint4 {
    pub ip: [u8; 4],
    pub port: u16,
}

#[derive(Clone, Copy, Default)]
pub struct SendVars {
    pub una: u32,
    pub nxt: u32,
    pub wnd: u16,
    pub iss: u32,
}

#[derive(Clone, Copy, Default)]
pub struct RecvVars {
    pub nxt: u32,
    pub wnd: u16,
    pub irs: u32,
}

#[derive(Clone, Copy)]
pub struct Tcb {
    pub local: Endpoint4,
    pub remote: Endpoint4,
    pub state: State,
    pub send: SendVars,
    pub recv: RecvVars,
}

impl Tcb {
    pub fn listen(local: Endpoint4) -> Self {
        Self {
            local,
            remote: Endpoint4 { ip: [0; 4], port: 0 },
            state: State::Listen,
            send: SendVars::default(),
            recv: RecvVars::default(),
        }
    }

    pub fn matches(&self, local: &Endpoint4, remote: &Endpoint4) -> bool {
        self.local.ip == local.ip
            && self.local.port == local.port
            && (self.state == State::Listen
                || (self.remote.ip == remote.ip && self.remote.port == remote.port))
    }
}
