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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    Listen = 0,
    SynSent = 1,
    SynReceived = 2,
    Established = 3,
    CloseWait = 4,
    FinWait1 = 5,
    FinWait2 = 6,
    Closing = 7,
    TimeWait = 8,
    LastAck = 9,
}

impl State {
    pub fn accepts_data(self) -> bool {
        matches!(self, Self::Established)
    }

    pub fn is_closing(self) -> bool {
        matches!(self, Self::FinWait1 | Self::FinWait2 | Self::Closing | Self::TimeWait | Self::LastAck)
    }
}
