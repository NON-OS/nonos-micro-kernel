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

use nonos_policy_proto::{Field, IPC_PAYLOAD_MAX, KIND_U8, OP_SET};

use super::error::IpcError;
use super::recv::recv_into;
use super::send::send;

pub fn op_set_u8(port: u32, field: Field, value: u8) -> Result<(), IpcError> {
    send(port, OP_SET, field as u32, KIND_U8, &[value])?;
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let _ = recv_into(&mut buf)?;
    Ok(())
}
