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

use super::kind::Granule;

pub const fn page_offset(addr: u64, granule: Granule) -> usize {
    (addr & ((granule.page_size() - 1) as u64)) as usize
}

pub const fn page_align_down(addr: u64, granule: Granule) -> u64 {
    addr & !((granule.page_size() - 1) as u64)
}

pub const fn page_align_up(addr: u64, granule: Granule) -> u64 {
    let mask = (granule.page_size() - 1) as u64;
    addr.saturating_add(mask) & !mask
}

pub const fn pages_needed(size: u64, granule: Granule) -> u64 {
    let page = granule.page_size() as u64;
    size.saturating_add(page - 1) / page
}
