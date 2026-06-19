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

use super::ffi;

#[cfg(all(target_arch = "x86_64", not(feature = "std")))]
mod dedicated {
    use super::ffi;
    use spin::Mutex;

    const VERIFY_STACK_SIZE: usize = 256 * 1024;

    #[repr(align(16))]
    struct VerifyStack([u8; VERIFY_STACK_SIZE]);

    static VERIFY_STACK: Mutex<VerifyStack> = Mutex::new(VerifyStack([0; VERIFY_STACK_SIZE]));

    #[repr(C)]
    struct VerifyCall {
        sig: *const u8,
        siglen: usize,
        msg: *const u8,
        msglen: usize,
        pk: *const u8,
    }

    unsafe extern "C" fn trampoline(call: *const VerifyCall) -> i32 {
        let call = unsafe { &*call };
        unsafe { ffi::verify(call.sig, call.siglen, call.msg, call.msglen, call.pk) }
    }

    pub(super) fn verify(
        sig: *const u8,
        siglen: usize,
        msg: *const u8,
        msglen: usize,
        pk: *const u8,
    ) -> i32 {
        let _irq = crate::interrupts::disable_interrupts_guard();
        let mut stack = VERIFY_STACK.lock();
        let top = stack.0.as_mut_ptr() as usize + VERIFY_STACK_SIZE;
        let call = VerifyCall { sig, siglen, msg, msglen, pk };
        unsafe { call_on_stack(&call, top) }
    }

    unsafe fn call_on_stack(call: &VerifyCall, stack_top: usize) -> i32 {
        let rc: i64;
        unsafe {
            core::arch::asm!(
                "mov r11, rsp",
                "mov rsp, {stack_top}",
                "and rsp, -16",
                "sub rsp, 16",
                "mov [rsp], r11",
                "call {entry}",
                "mov rsp, [rsp]",
                stack_top = in(reg) stack_top,
                entry = in(reg) trampoline,
                in("rdi") call as *const VerifyCall,
                lateout("rax") rc,
                lateout("r11") _,
                clobber_abi("C"),
            );
        }
        rc as i32
    }
}

#[cfg(any(not(target_arch = "x86_64"), feature = "std"))]
mod dedicated {
    use super::ffi;

    pub(super) fn verify(
        sig: *const u8,
        siglen: usize,
        msg: *const u8,
        msglen: usize,
        pk: *const u8,
    ) -> i32 {
        unsafe { ffi::verify(sig, siglen, msg, msglen, pk) }
    }
}

pub(super) fn verify(
    sig: *const u8,
    siglen: usize,
    msg: *const u8,
    msglen: usize,
    pk: *const u8,
) -> i32 {
    dedicated::verify(sig, siglen, msg, msglen, pk)
}
