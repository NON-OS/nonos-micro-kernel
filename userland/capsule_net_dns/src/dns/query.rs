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

use super::header::{FLAG_RD, HDR_LEN};
use super::name::{encode as encode_name, NameError};
use super::types::{CLASS_IN, TYPE_A, TYPE_AAAA};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    OutputTooSmall,
    NameInvalid,
}

impl From<NameError> for BuildError {
    fn from(e: NameError) -> Self {
        match e {
            NameError::TooLong | NameError::LabelTooLong => Self::NameInvalid,
            _ => Self::OutputTooSmall,
        }
    }
}

pub fn build_a_query(xid: u16, name: &str, out: &mut [u8]) -> Result<usize, BuildError> {
    build_query(xid, name, TYPE_A, out)
}

pub fn build_aaaa_query(xid: u16, name: &str, out: &mut [u8]) -> Result<usize, BuildError> {
    build_query(xid, name, TYPE_AAAA, out)
}

fn build_query(xid: u16, name: &str, qtype: u16, out: &mut [u8]) -> Result<usize, BuildError> {
    if out.len() < HDR_LEN {
        return Err(BuildError::OutputTooSmall);
    }
    out[0..2].copy_from_slice(&xid.to_be_bytes());
    out[2..4].copy_from_slice(&FLAG_RD.to_be_bytes());
    out[4..6].copy_from_slice(&1u16.to_be_bytes());
    out[6..8].copy_from_slice(&0u16.to_be_bytes());
    out[8..10].copy_from_slice(&0u16.to_be_bytes());
    out[10..12].copy_from_slice(&0u16.to_be_bytes());
    let name_len = encode_name(name, &mut out[HDR_LEN..])?;
    let qpos = HDR_LEN + name_len;
    if qpos + 4 > out.len() {
        return Err(BuildError::OutputTooSmall);
    }
    out[qpos..qpos + 2].copy_from_slice(&qtype.to_be_bytes());
    out[qpos + 2..qpos + 4].copy_from_slice(&CLASS_IN.to_be_bytes());
    Ok(qpos + 4)
}
