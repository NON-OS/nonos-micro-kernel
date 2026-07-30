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

use super::super::error::PageInfoResult;
use super::super::types::{PageFlags, PageInfo, PageStatsSnapshot};
use super::state::{PAGE_INFO_MANAGER, PAGE_STATS};
use crate::memory::addr::{PhysAddr, VirtAddr};
use core::sync::atomic::Ordering;

pub fn get_timestamp() -> u64 {
    crate::arch::read_time_counter()
}

pub fn init() -> PageInfoResult<()> {
    PAGE_INFO_MANAGER.lock().init()
}
pub fn add_page(pa: PhysAddr, va: Option<VirtAddr>, flags: PageFlags) -> PageInfoResult<()> {
    PAGE_INFO_MANAGER.lock().add_page(pa, va, flags)
}
pub fn remove_page(pa: PhysAddr) -> PageInfoResult<()> {
    PAGE_INFO_MANAGER.lock().remove_page(pa)
}
pub fn get_page_info(pa: PhysAddr) -> Option<PageInfo> {
    PAGE_INFO_MANAGER.lock().get_page_info(pa)
}
pub fn update_page_flags(pa: PhysAddr, flags: PageFlags) -> PageInfoResult<()> {
    PAGE_INFO_MANAGER.lock().update_flags(pa, flags)
}
pub fn increment_ref_count(pa: PhysAddr) -> PageInfoResult<u32> {
    PAGE_INFO_MANAGER.lock().increment_ref_count(pa)
}
pub fn decrement_ref_count(pa: PhysAddr) -> PageInfoResult<u32> {
    PAGE_INFO_MANAGER.lock().decrement_ref_count(pa)
}

pub fn get_page_stats() -> (usize, usize, usize, usize, u64) {
    (
        PAGE_STATS.total_pages.load(Ordering::Relaxed),
        PAGE_STATS.mapped_pages.load(Ordering::Relaxed),
        PAGE_STATS.dirty_pages.load(Ordering::Relaxed),
        PAGE_STATS.locked_pages.load(Ordering::Relaxed),
        PAGE_STATS.page_accesses.load(Ordering::Relaxed),
    )
}

pub fn get_stats_snapshot() -> PageStatsSnapshot {
    PageStatsSnapshot {
        total_pages: PAGE_STATS.total_pages.load(Ordering::Relaxed),
        mapped_pages: PAGE_STATS.mapped_pages.load(Ordering::Relaxed),
        dirty_pages: PAGE_STATS.dirty_pages.load(Ordering::Relaxed),
        locked_pages: PAGE_STATS.locked_pages.load(Ordering::Relaxed),
        page_accesses: PAGE_STATS.page_accesses.load(Ordering::Relaxed),
    }
}

pub fn page_count() -> usize {
    PAGE_STATS.total_pages.load(Ordering::Relaxed)
}
pub fn is_initialized() -> bool {
    PAGE_INFO_MANAGER.lock().initialized
}
