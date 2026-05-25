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

use nonos_policy_proto::{
    kind_of, Field, IPC_PAYLOAD_MAX, KIND_BOOL, KIND_I8, KIND_STR, KIND_U8, OP_GET,
};

use crate::settings::state::cache::{FieldValue, STRING_CAP};

use super::error::IpcError;
use super::recv::recv_into;
use super::send::send;

pub fn op_get(port: u32, field: Field) -> Result<FieldValue, IpcError> {
    let kind = kind_of(field);
    send(port, OP_GET, field as u32, kind, &[])?;
    let mut buf = [0u8; IPC_PAYLOAD_MAX];
    let reply = recv_into(&mut buf)?;
    if reply.header.kind != kind {
        return Err(IpcError::KindMismatch);
    }
    Ok(match kind {
        KIND_BOOL => bool_value(reply.payload)?,
        KIND_U8 => u8_value(reply.payload)?,
        KIND_I8 => i8_value(reply.payload)?,
        KIND_STR => str_value(reply.payload)?,
        _ => return Err(IpcError::KindMismatch),
    })
}

fn bool_value(p: &[u8]) -> Result<FieldValue, IpcError> {
    if p.is_empty() {
        return Err(IpcError::ShortReply);
    }
    Ok(FieldValue::Bool(p[0] != 0))
}

fn u8_value(p: &[u8]) -> Result<FieldValue, IpcError> {
    if p.is_empty() {
        return Err(IpcError::ShortReply);
    }
    Ok(FieldValue::U8(p[0]))
}

fn i8_value(p: &[u8]) -> Result<FieldValue, IpcError> {
    if p.is_empty() {
        return Err(IpcError::ShortReply);
    }
    Ok(FieldValue::I8(p[0] as i8))
}

fn str_value(p: &[u8]) -> Result<FieldValue, IpcError> {
    let mut bytes = [0u8; STRING_CAP];
    let n = core::cmp::min(p.len(), STRING_CAP);
    bytes[..n].copy_from_slice(&p[..n]);
    Ok(FieldValue::Str { bytes, len: n })
}
