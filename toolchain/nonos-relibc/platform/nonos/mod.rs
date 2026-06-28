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

    fn faccessat(_fd: c_int, _path: CStr, _amode: c_int, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn chdir(_path: CStr) -> Result<()> { Err(Errno(ENOSYS)) }
    fn close(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn dup2(_fildes: c_int, _fildes2: c_int) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn fchdir(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fchmodat(_dirfd: c_int, _path: Option<CStr>, _mode: mode_t, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fchownat(_fildes: c_int, _path: CStr, _owner: uid_t, _group: gid_t, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fdatasync(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn flock(_fd: c_int, _operation: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fstatat(_fildes: c_int, _path: Option<CStr>, _buf: Out<stat>, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fstatvfs(_fildes: c_int, _buf: Out<statvfs>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fcntl(_fildes: c_int, _cmd: c_int, _arg: c_ulonglong) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn fpath(_fildes: c_int, _out: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn fsync(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn ftruncate(_fildes: c_int, _length: off_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn utimensat(_dirfd: c_int, _path: CStr, _times: *const timespec, _flag: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getcwd(_buf: Out<[u8]>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getdents(_fd: c_int, _buf: &mut [u8], _opaque_offset: u64) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn dir_seek(_fd: c_int, _opaque_offset: u64) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn dent_reclen_offset(_this_dent: &[u8], _offset: usize) -> Option<(u16, u64)> { None }
    fn linkat(_fd1: c_int, _oldpath: CStr, _fd2: c_int, _newpath: CStr, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn lseek(_fildes: c_int, _offset: off_t, _whence: c_int) -> Result<off_t> { Err(Errno(ENOSYS)) }
    fn mkdirat(_fildes: c_int, _path: CStr, _mode: mode_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn mkfifoat(_dir_fd: c_int, _path: CStr, _mode: mode_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn mknodat(_fildes: c_int, _path: CStr, _mode: mode_t, _dev: dev_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn openat(_dirfd: c_int, _path: CStr, _oflag: c_int, _mode: mode_t) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn pipe2(_fildes: Out<[c_int; 2]>, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn posix_fallocate(_fd: c_int, _offset: u64, _length: NonZeroU64) -> Result<()> { Err(Errno(ENOSYS)) }
    fn posix_getdents(_fildes: c_int, _buf: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn read(_fildes: c_int, _buf: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn pread(_fildes: c_int, _buf: &mut [u8], _offset: off_t) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn pwrite(_fildes: c_int, _buf: &[u8], _offset: off_t) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn readlinkat(_dirfd: c_int, _pathname: CStr, _out: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn renameat2(_old_dir: c_int, _old_path: CStr, _new_dir: c_int, _new_path: CStr, _flags: c_uint) -> Result<()> { Err(Errno(ENOSYS)) }
    fn symlinkat(_path1: CStr, _fd: c_int, _path2: CStr) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sync() -> Result<()> { Err(Errno(ENOSYS)) }
    fn unlinkat(_fd: c_int, _path: CStr, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn mlock(_addr: *const c_void, _len: usize) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn mlockall(_flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn mremap(_addr: *mut c_void, _len: usize, _new_len: usize, _flags: c_int, _args: *mut c_void) -> Result<*mut c_void> { Err(Errno(ENOSYS)) }
    unsafe fn mprotect(_addr: *mut c_void, _len: usize, _prot: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn msync(_addr: *mut c_void, _len: usize, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn munlock(_addr: *const c_void, _len: usize) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn madvise(_addr: *mut c_void, _len: usize, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn munlockall() -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn nanosleep(_rqtp: *const timespec, _rmtp: *mut timespec) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getpagesize() -> usize { 4096 }

    fn verify() -> bool {
        true
    }
}
