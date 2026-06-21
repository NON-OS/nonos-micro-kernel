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

//! `MkPciConfigWrite` orchestration.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::constants::CFG_COMMAND;

use super::allowlist::validate;
use super::ownership::resolve;
use super::types::{PciWriteError, PciWriteRequest, WriteAction};

pub fn write(pid: u32, req: PciWriteRequest) -> Result<(), PciWriteError> {
    let handle = resolve(pid, &req)?;
    let cfg = ConfigSpace::new(handle.address);
    let current = cfg.read16(req.offset as u16).map_err(|_| PciWriteError::PlatformError)?;
    let action = validate(&req, handle.msix.as_ref(), current)?;
    apply(&cfg, action)
}

fn apply(cfg: &ConfigSpace, action: WriteAction) -> Result<(), PciWriteError> {
    match action {
        WriteAction::Command(value) => {
            cfg.write16(CFG_COMMAND, value).map_err(|_| PciWriteError::PlatformError)
        }
        WriteAction::MsixControl { offset, value } => {
            cfg.write16(offset, value).map_err(|_| PciWriteError::PlatformError)
        }
    }
}
