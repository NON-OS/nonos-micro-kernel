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

use super::definition::Context;
use super::restore_asm::context_restore_asm;

const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;
const KERNEL_SPACE_MIN: u64 = 0xFFFF_8000_0000_0000;
const RFLAGS_PRIVILEGED_MASK: u64 = 0x0000_0000_001F_7500;
const RFLAGS_RESERVED_SET: u64 = 0x0000_0000_0000_0002;
const RFLAGS_IF: u64 = 0x0000_0000_0000_0200;
impl Context {
    #[inline]
    fn is_user_space_addr(addr: u64) -> bool {
        addr <= USER_SPACE_MAX
    }

    #[inline]
    pub(super) fn sanitize_rflags(rflags: u64) -> u64 {
        (rflags & !RFLAGS_PRIVILEGED_MASK) | RFLAGS_RESERVED_SET | RFLAGS_IF
    }

    fn is_canonical(addr: u64) -> bool {
        addr <= USER_SPACE_MAX || addr >= KERNEL_SPACE_MIN
    }

    pub fn validate(&self) -> Result<(), &'static str> {
        if !Self::is_canonical(self.rip) {
            return Err("RIP in non-canonical address range");
        }
        if !Self::is_canonical(self.rsp) {
            return Err("RSP in non-canonical address range");
        }
        if self.rsp == 0 {
            return Err("RSP is null");
        }
        Ok(())
    }

    pub fn validate_userspace(&self) -> Result<(), &'static str> {
        self.validate()?;
        if !Self::is_user_space_addr(self.rip) {
            return Err("RIP not in user space");
        }
        if !Self::is_user_space_addr(self.rsp) {
            return Err("RSP not in user space");
        }
        Ok(())
    }

    pub fn restore(&self) -> ! {
        if let Err(e) = self.validate() {
            crate::sys::serial::println(b"[FATAL] Context restore failed");
            crate::sys::serial::println(e.as_bytes());
            crate::arch::halt_loop()
        }
        let mut safe_ctx = *self;
        safe_ctx.rflags = Self::sanitize_rflags(safe_ctx.rflags);
        super::save::set_restored_flag();
        context_restore_asm(&safe_ctx as *const Context)
    }
}
