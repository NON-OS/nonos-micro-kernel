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

use nonos_libc::{mk_device_release, BAR_KIND_MMIO, BAR_KIND_PIO};

use super::grant_mmio::grant_mmio;
use super::grant_pio::grant_pio;
use super::register_grant::RegisterGrant;
use crate::discover::Found;

pub fn map(dev: Found, claim_epoch: u64) -> Result<RegisterGrant, &'static str> {
    match dev.register_kind {
        BAR_KIND_MMIO => grant_mmio(dev, claim_epoch),
        BAR_KIND_PIO => grant_pio(dev, claim_epoch),
        _ => {
            let _ = mk_device_release(dev.device_id);
            Err("unsupported register bar")
        }
    }
}
