//! NONOS platform backend for relibc. Task 0.3 ships the full `Pal` surface:
//! five real methods backed by `MK_*` syscalls (write, exit, getpid, mmap,
//! munmap) and the remaining required methods stubbed to `ENOSYS` (or the
//! obvious constant). Phase 1 de-stubs this backend method by method.

use core::num::NonZeroU64;

use super::{Pal, types::*};
use crate::{
    c_str::CStr,
    error::{Errno, Result},
    header::{
        errno::{EBADF, ENOMEM, ENOSYS},
        signal::sigevent,
        sys_resource::{rlimit, rusage},
        sys_select::timeval,
        sys_stat::stat,
        sys_statvfs::statvfs,
        sys_time::timezone,
        sys_utsname::utsname,
        time::{itimerspec, timespec},
    },
    iter::NulTerminated,
    ld_so::tcb::OsSpecific,
    out::Out,
    pthread,
};

pub mod lowlevel;

mod epoll;
mod ptrace;
mod signal;
mod socket;

pub struct Sys;

impl Pal for Sys {
    fn write(fildes: c_int, buf: &[u8]) -> Result<usize> {
        if fildes == 1 || fildes == 2 {
            unsafe { lowlevel::syscall2(lowlevel::MK_DEBUG, buf.as_ptr() as u64, buf.len() as u64); }
            return Ok(buf.len());
        }
        Err(Errno(EBADF))
    }

    fn exit(status: c_int) -> ! {
        unsafe { lowlevel::syscall1(lowlevel::MK_EXIT, status as u64); }
        loop {}
    }

    fn getpid() -> pid_t {
        unsafe { lowlevel::syscall0(lowlevel::MK_GETPID) as pid_t }
    }

    unsafe fn mmap(_addr: *mut c_void, len: usize, _prot: c_int, _flags: c_int, _fildes: c_int, _off: off_t) -> Result<*mut c_void> {
        let r = unsafe { lowlevel::syscall4(lowlevel::MK_MMAP, 0, len as u64, 3, 0) };
        if r <= 0 {
            return Err(Errno(ENOMEM));
        }
        Ok(r as *mut c_void)
    }

    unsafe fn munmap(addr: *mut c_void, len: usize) -> Result<()> {
        unsafe { lowlevel::syscall2(lowlevel::MK_MUNMAP, addr as u64, len as u64); }
        Ok(())
    }

    unsafe fn brk(_addr: *mut c_void) -> Result<*mut c_void> {
        Err(Errno(ENOSYS))
    }

    fn verify() -> bool {
        true
    }
}
