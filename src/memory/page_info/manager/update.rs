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

use super::super::error::{PageInfoError, PageInfoResult};
use super::super::types::PageFlags;
use super::api::get_timestamp;
use super::state::{PageInfoManager, PAGE_STATS};
use crate::memory::addr::PhysAddr;
use crate::memory::layout;

impl PageInfoManager {
    pub(super) fn update_flags(&mut self, pa: PhysAddr, flags: PageFlags) -> PageInfoResult<()> {
        let page_num = pa.as_u64() / layout::PAGE_SIZE as u64;
        if let Some(info) = self.pages.get_mut(&page_num) {
            let old_flags = info.flags;
            info.flags = flags;
            info.last_access = get_timestamp();

            if old_flags.contains(PageFlags::DIRTY) != flags.contains(PageFlags::DIRTY) {
                if flags.contains(PageFlags::DIRTY) {
                    PAGE_STATS.increment_dirty();
                } else {
                    PAGE_STATS.decrement_dirty();
                }
            }
            if old_flags.contains(PageFlags::LOCKED) != flags.contains(PageFlags::LOCKED) {
                if flags.contains(PageFlags::LOCKED) {
                    PAGE_STATS.increment_locked();
                } else {
                    PAGE_STATS.decrement_locked();
                }
            }
            PAGE_STATS.record_access();
            Ok(())
        } else {
            Err(PageInfoError::PageNotFound)
        }
    }

    pub(super) fn increment_ref_count(&mut self, pa: PhysAddr) -> PageInfoResult<u32> {
        let page_num = pa.as_u64() / layout::PAGE_SIZE as u64;
        if let Some(info) = self.pages.get_mut(&page_num) {
            info.ref_count = info.ref_count.saturating_add(1);
            info.last_access = get_timestamp();
            Ok(info.ref_count)
        } else {
            Err(PageInfoError::PageNotFound)
        }
    }

    pub(super) fn decrement_ref_count(&mut self, pa: PhysAddr) -> PageInfoResult<u32> {
        let page_num = pa.as_u64() / layout::PAGE_SIZE as u64;
        if let Some(info) = self.pages.get_mut(&page_num) {
            match super::refcount::dec_checked(info.ref_count) {
                Some(next) => {
                    info.ref_count = next;
                    info.last_access = get_timestamp();
                    Ok(next)
                }
                None => Err(PageInfoError::RefCountUnderflow),
            }
        } else {
            Err(PageInfoError::PageNotFound)
        }
    }
}
