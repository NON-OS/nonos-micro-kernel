// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::constants::{
    AVAIL_OFFSET, BUFFER_SIZE, DESC_OFFSET, QUEUE_SIZE, USED_OFFSET, VIRTQ_DESC_F_WRITE,
};
use super::core::RngQueue;
use super::types::{VirtqAvail, VirtqDesc, VirtqUsed};
use core::ptr;
use core::sync::atomic::Ordering;

impl RngQueue {
    #[inline]
    fn desc_ptr(&self) -> *mut VirtqDesc {
        (self.vq_base + DESC_OFFSET) as *mut VirtqDesc
    }
    #[inline]
    fn avail_ptr(&self) -> *mut VirtqAvail {
        (self.vq_base + AVAIL_OFFSET) as *mut VirtqAvail
    }
    #[inline]
    fn used_ptr(&self) -> *mut VirtqUsed {
        (self.vq_base + USED_OFFSET) as *mut VirtqUsed
    }

    pub(crate) fn request_random(&mut self, len: usize) -> Result<(), &'static str> {
        let len = len.min(BUFFER_SIZE);
        self.pending_len = len;
        unsafe {
            ptr::write_bytes(self.buf_base as *mut u8, 0, BUFFER_SIZE);
            let desc = &mut *self.desc_ptr();
            desc.addr = self.buf_base as u64;
            desc.len = len as u32;
            desc.flags = VIRTQ_DESC_F_WRITE;
            desc.next = 0;
            let avail = &mut *self.avail_ptr();
            let idx = self.next_avail_idx;
            avail.ring[(idx % QUEUE_SIZE) as usize] = 0;
            core::sync::atomic::fence(Ordering::SeqCst);
            self.next_avail_idx = self.next_avail_idx.wrapping_add(1);
            avail.idx = self.next_avail_idx;
        }
        self.kick();
        Ok(())
    }

    pub(crate) fn has_completed(&self) -> bool {
        unsafe {
            let used = &*self.used_ptr();
            used.idx != self.last_used_idx
        }
    }

    pub(crate) fn get_received_bytes(&mut self, buf: &mut [u8]) -> usize {
        let received_len = unsafe {
            let used = &*self.used_ptr();
            if used.idx == self.last_used_idx {
                return 0;
            }
            let used_elem = &used.ring[(self.last_used_idx % QUEUE_SIZE) as usize];
            self.last_used_idx = self.last_used_idx.wrapping_add(1);
            used_elem.len as usize
        };
        let copy_len = received_len.min(buf.len()).min(self.pending_len);
        unsafe {
            ptr::copy_nonoverlapping(self.buf_base as *const u8, buf.as_mut_ptr(), copy_len);
        }
        self.pending_len = 0;
        copy_len
    }

    fn kick(&self) {
        core::sync::atomic::fence(Ordering::SeqCst);
        if self.notify_port != 0 {
            // SAFETY: the probe claimed this port for this device.
            unsafe {
                crate::sys::io::outw(self.notify_port, 0);
            }
        } else if self.notify_mmio != 0 {
            unsafe {
                ptr::write_volatile(self.notify_mmio as *mut u16, 0);
            }
        }
    }
}
