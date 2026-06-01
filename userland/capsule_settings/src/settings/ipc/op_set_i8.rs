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

use nonos_policy_proto::{Field, IPC_PAYLOAD_MAX, KIND_I8, OP_SET};

use super::call::call;
use super::error::IpcError;

pub fn op_set_i8(port: u32, field: Field, value: i8) -> Result<(), IpcError> {
    let mut rx = [0u8; IPC_PAYLOAD_MAX];
    let _ = call(port, OP_SET, field as u32, KIND_I8, &[value as u8], &mut rx)?;
    Ok(())
}
