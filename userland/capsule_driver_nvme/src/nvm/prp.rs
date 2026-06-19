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

use super::constants::PAGE;
use super::queue::IoQueue;

pub(super) fn build_prp(io: &IoQueue, bytes: u64) -> (u64, u64) {
    let data_phys = io.data.device_addr();
    if bytes <= PAGE {
        return (data_phys, 0);
    }
    if bytes <= 2 * PAGE {
        return (data_phys, data_phys + PAGE);
    }
    let npages = (bytes + PAGE - 1) / PAGE;
    let list_va = io.prp_list.user_va();
    for i in 0..(npages - 1) {
        let entry = data_phys + PAGE * (i + 1);
        unsafe { core::ptr::write_volatile((list_va + i * 8) as *mut u64, entry) };
    }
    (data_phys, io.prp_list.device_addr())
}
