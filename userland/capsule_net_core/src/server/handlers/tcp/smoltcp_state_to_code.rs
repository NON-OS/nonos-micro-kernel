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

use smoltcp::socket::tcp;

pub fn smoltcp_state_to_code(s: tcp::State) -> u8 {
    match s {
        tcp::State::Listen => 0,
        tcp::State::SynSent => 1,
        tcp::State::SynReceived => 2,
        tcp::State::Established => 3,
        tcp::State::CloseWait => 4,
        tcp::State::FinWait1 => 5,
        tcp::State::FinWait2 => 6,
        tcp::State::Closing => 7,
        tcp::State::TimeWait => 8,
        tcp::State::LastAck => 9,
        tcp::State::Closed => 0xFF,
    }
}
