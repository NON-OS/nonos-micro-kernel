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

use crate::device::budget;
use crate::device::rx;
use crate::device::types::{NicRxToken, NicTxToken};

pub fn receive(port: u32) -> Option<(NicRxToken, NicTxToken)> {
    if !budget::poll_open() {
        return None;
    }
    let frame = rx::poll_frame(port)?;
    Some((NicRxToken(frame), NicTxToken { port }))
}
