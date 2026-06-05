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

use super::types::ProbeResult;
use super::visitor::visit_record;
use super::wire::*;
use crate::protocol::{E_INVAL, E_NO_MSC};

pub fn parse_config(raw: &[u8]) -> Result<ProbeResult, i32> {
    if raw.len() < 9 || raw[1] != DESC_CONFIGURATION {
        return Err(E_INVAL);
    }
    let total = u16::from_le_bytes([raw[2], raw[3]]) as usize;
    if total < 9 || total > raw.len() {
        return Err(E_INVAL);
    }
    let mut out = ProbeResult::empty();
    let mut current = None;
    let mut pos = 0usize;
    while pos + 2 <= total {
        let len = raw[pos] as usize;
        if len < 2 || pos + len > total {
            return Err(E_INVAL);
        }
        visit_record(&raw[pos..pos + len], &mut current, &mut out);
        pos += len;
    }
    if out.count == 0 {
        Err(E_NO_MSC)
    } else {
        Ok(out)
    }
}
