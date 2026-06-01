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

//! Ownership + handle resolution for `MkPciConfigWrite`. Split out
//! of `write.rs` so the host test crate can drive the four early
//! rejection paths (`NotClaimed` for missing claim, `NotClaimed`
//! for wrong pid, `StaleEpoch`, `NoDeviceHandle`) against the real
//! production code without needing to fake the PCI config-space
//! port I/O the rest of `write.rs` does.

use crate::hardware::broker::claim;
use crate::hardware::broker::pci_index::{self, PciHandle};

use super::types::{PciReadError, PciReadRequest, PciWriteError, PciWriteRequest};

pub(super) fn resolve(pid: u32, req: &PciWriteRequest) -> Result<PciHandle, PciWriteError> {
    let claim = claim::lookup(req.device_id).ok_or(PciWriteError::NotClaimed)?;
    if claim.pid != pid {
        return Err(PciWriteError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(PciWriteError::StaleEpoch);
    }
    pci_index::lookup(req.device_id).ok_or(PciWriteError::NoDeviceHandle)
}

pub(super) fn resolve_read(pid: u32, req: &PciReadRequest) -> Result<PciHandle, PciReadError> {
    let claim = claim::lookup(req.device_id).ok_or(PciReadError::NotClaimed)?;
    if claim.pid != pid {
        return Err(PciReadError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(PciReadError::StaleEpoch);
    }
    pci_index::lookup(req.device_id).ok_or(PciReadError::NoDeviceHandle)
}
