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

use crate::drivers::pci::config::ConfigSpace;

use super::ownership::resolve_read;
use super::types::{PciReadError, PciReadRequest};

const CONFIG_LIMIT: u32 = 256;

pub fn read(pid: u32, req: PciReadRequest) -> Result<u32, PciReadError> {
    validate(&req)?;
    let handle = resolve_read(pid, &req)?;
    let cfg = ConfigSpace::new(handle.address);
    match req.width {
        1 => cfg.read8(req.offset as u16).map(|v| v as u32),
        2 => cfg.read16(req.offset as u16).map(|v| v as u32),
        4 => cfg.read32(req.offset as u16),
        _ => return Err(PciReadError::WidthNotAllowed),
    }
    .map_err(|_| PciReadError::PlatformError)
}

fn validate(req: &PciReadRequest) -> Result<(), PciReadError> {
    if req.width != 1 && req.width != 2 && req.width != 4 {
        return Err(PciReadError::WidthNotAllowed);
    }
    if req.offset.checked_add(req.width).filter(|end| *end <= CONFIG_LIMIT).is_none() {
        return Err(PciReadError::OffsetNotAllowed);
    }
    if req.offset & (req.width - 1) != 0 {
        return Err(PciReadError::OffsetNotAllowed);
    }
    Ok(())
}
