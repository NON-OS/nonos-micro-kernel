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

use nonos_libc::{mk_device_release, mk_mmio_unmap, MmioMapOut};

use super::state::RegisterGrant;
use crate::discover::Found;

pub fn rollback_one(
    dev: Found,
    common: MmioMapOut,
    err: &'static str,
) -> Result<RegisterGrant, &'static str> {
    if mk_mmio_unmap(common.grant_id) < 0 || mk_device_release(dev.device_id) < 0 {
        return Err("virtio-gpu: modern rollback failed");
    }
    Err(err)
}

pub fn rollback_two(
    dev: Found,
    common: MmioMapOut,
    notify: MmioMapOut,
    err: &'static str,
) -> Result<RegisterGrant, &'static str> {
    if mk_mmio_unmap(common.grant_id) < 0 || mk_mmio_unmap(notify.grant_id) < 0 {
        return Err("virtio-gpu: modern rollback failed");
    }
    if mk_device_release(dev.device_id) < 0 {
        return Err("virtio-gpu: modern release rollback failed");
    }
    Err(err)
}
