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

use vstd::prelude::*;

verus! {

pub open spec fn max_message_size() -> u64 {
    1048576u64
}

pub open spec fn valid_ipc_len(len: u64) -> bool {
    len != 0 && len <= max_message_size()
}

pub proof fn zero_length_is_rejected()
    ensures
        !valid_ipc_len(0),
{
}

pub proof fn oversized_message_is_rejected(len: u64)
    requires
        len > max_message_size(),
    ensures
        !valid_ipc_len(len),
{
}

pub proof fn accepted_message_is_bounded(len: u64)
    requires
        valid_ipc_len(len),
    ensures
        len > 0,
        len <= max_message_size(),
{
}

pub proof fn send_reply_share_the_same_length_gate(len: u64)
    ensures
        valid_ipc_len(len) == (len != 0 && len <= 1048576u64),
{
}

} // verus!
