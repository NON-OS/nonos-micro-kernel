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

//! The device table the test presents, and the interrupt lines the driver
//! acknowledged against it.

use std::cell::RefCell;

use super::types::DeviceRecord;

thread_local! {
    static DEVICES: RefCell<Vec<DeviceRecord>> = const { RefCell::new(Vec::new()) };
    static ACKED: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// What the broker lists on this thread from now on.
pub fn present(devices: &[DeviceRecord]) {
    DEVICES.with(|d| *d.borrow_mut() = devices.to_vec());
    ACKED.with(|a| a.borrow_mut().clear());
}

pub(super) fn acknowledge(grant_id: u64) {
    ACKED.with(|a| a.borrow_mut().push(grant_id));
}

/// The interrupt grants acknowledged so far, in order.
pub fn acked() -> Vec<u64> {
    ACKED.with(|a| a.borrow().clone())
}

pub fn mk_device_list(_class: u32, buf: *mut DeviceRecord, count: u64) -> i64 {
    DEVICES.with(|d| {
        let devices = d.borrow();
        let n = devices.len().min(count as usize);
        for (i, r) in devices.iter().take(n).enumerate() {
            /*
             * SAFETY: the driver hands a buffer of `count` records, as the
             * real call requires, and `i` stays below it.
             */
            unsafe { *buf.add(i) = *r };
        }
        devices.len() as i64
    })
}
