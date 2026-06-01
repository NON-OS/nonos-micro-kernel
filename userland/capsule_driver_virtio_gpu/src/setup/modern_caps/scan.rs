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

use super::read::{u32_at, u8_at};
use super::types::{ModernCaps, Region};

const CAP_PTR: u32 = 0x34;
const PCI_CAP_VENDOR: u8 = 0x09;
const CFG_COMMON: u8 = 1;
const CFG_NOTIFY: u8 = 2;
const CFG_DEVICE: u8 = 4;

pub fn read(device_id: u64, epoch: u64) -> Result<ModernCaps, &'static str> {
    crate::debug::marker(b"caps: read CAP_PTR");
    let mut common = None;
    let mut notify = None;
    let mut device = None;
    let mut multiplier = 0;
    let mut ptr = u8_at(device_id, epoch, CAP_PTR)? as u32 & 0xFC;
    crate::debug::marker(b"caps: walk");
    for _ in 0..48 {
        if ptr < 0x40 {
            break;
        }
        scan_one(device_id, epoch, ptr, &mut common, &mut notify, &mut device, &mut multiplier)?;
        ptr = u8_at(device_id, epoch, ptr + 1)? as u32 & 0xFC;
    }
    crate::debug::marker(b"caps: walk done");
    Ok(ModernCaps {
        common: common.ok_or("virtio-gpu: common cfg missing")?,
        notify: notify.ok_or("virtio-gpu: notify cfg missing")?,
        device: device.ok_or("virtio-gpu: device cfg missing")?,
        notify_multiplier: multiplier,
    })
}

fn scan_one(
    device_id: u64,
    epoch: u64,
    ptr: u32,
    common: &mut Option<Region>,
    notify: &mut Option<Region>,
    device: &mut Option<Region>,
    multiplier: &mut u32,
) -> Result<(), &'static str> {
    if u8_at(device_id, epoch, ptr)? != PCI_CAP_VENDOR {
        return Ok(());
    }
    let region = Region {
        bar: u8_at(device_id, epoch, ptr + 4)?,
        offset: u32_at(device_id, epoch, ptr + 8)?,
        length: u32_at(device_id, epoch, ptr + 12)?,
    };
    match u8_at(device_id, epoch, ptr + 3)? {
        CFG_COMMON => *common = Some(region),
        CFG_NOTIFY => {
            *notify = Some(region);
            *multiplier = u32_at(device_id, epoch, ptr + 16)?;
        }
        CFG_DEVICE => *device = Some(region),
        _ => {}
    }
    Ok(())
}
