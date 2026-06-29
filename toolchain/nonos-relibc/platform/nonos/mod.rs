//! NONOS platform backend for relibc. Task 0.3 ships the full `Pal` surface:
//! five real methods backed by `MK_*` syscalls (write, exit, getpid, mmap,
//! munmap) and the remaining required methods stubbed to `ENOSYS` (or the
//! obvious constant). Phase 1 de-stubs this backend method by method.

use alloc::vec::Vec;
use core::num::NonZeroU64;

use super::{Pal, types::*};
use crate::{
    c_str::CStr,
    error::{Errno, Result},
    header::{
        errno::{EBADF, EIO, EINVAL, EMFILE, ENOMEM, ENOSYS},
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
mod fs;

mod epoll;
mod ptrace;
mod signal;
mod socket;

fn now_ms() -> i64 {
    let r = unsafe { lowlevel::syscall0(lowlevel::MK_TIME_MILLIS) };
    if r < 0 { 0 } else { r }
}

pub struct Sys;

impl Pal for Sys {
    fn write(fildes: c_int, buf: &[u8]) -> Result<usize> {
        if fildes == 1 || fildes == 2 {
            unsafe { lowlevel::syscall2(lowlevel::MK_DEBUG, buf.as_ptr() as u64, buf.len() as u64); }
            return Ok(buf.len());
        }
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        if buf.len() > 65536 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = alloc::vec![0u8; 8 + buf.len()];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        payload[8..].copy_from_slice(buf);
        let mut resp = [0u8; 28];
        let (status, _) = fs::vfs_call(fs::OP_WRITE, &payload, &mut resp)?;
        if status < 0 { return Err(Errno(-status)); }
        Ok(u32::from_le_bytes([resp[24], resp[25], resp[26], resp[27]]) as usize)
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
    fn close(fildes: c_int) -> Result<()> {
        if fildes <= 2 { return Ok(()); }
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        let pid = Self::getpid() as u32;
        let mut payload = [0u8; 8];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        let mut resp = [0u8; 24];
        let (status, _) = fs::vfs_call(fs::OP_CLOSE, &payload, &mut resp)?;
        fs::fd_free(fildes);
        if status < 0 { Err(Errno(-status)) } else { Ok(()) }
    }
    fn dup2(fildes: c_int, fildes2: c_int) -> Result<c_int> {
        if fildes == fildes2 { return Ok(fildes2); }
        if fildes2 < 3 { return Err(Errno(EBADF)); }
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        if !fs::fd_set(fildes2, vfs_fd) { return Err(Errno(EBADF)); }
        Ok(fildes2)
    }
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
    fn read(fildes: c_int, buf: &mut [u8]) -> Result<usize> {
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        let count = (buf.len() as u32).min(65536);
        let pid = Self::getpid() as u32;
        let mut payload = [0u8; 12];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        payload[8..12].copy_from_slice(&count.to_le_bytes());
        let mut resp = alloc::vec![0u8; 24 + count as usize];
        let (status, n) = fs::vfs_call(fs::OP_READ, &payload, &mut resp)?;
        if status < 0 { return Err(Errno(-status)); }
        buf[..n].copy_from_slice(&resp[24..24 + n]);
        Ok(n)
    }
    fn pread(_fildes: c_int, _buf: &mut [u8], _offset: off_t) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn pwrite(_fildes: c_int, _buf: &[u8], _offset: off_t) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn readlinkat(_dirfd: c_int, _pathname: CStr, _out: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn renameat2(_old_dir: c_int, _old_path: CStr, _new_dir: c_int, _new_path: CStr, _flags: c_uint) -> Result<()> { Err(Errno(ENOSYS)) }
    fn symlinkat(_path1: CStr, _fd: c_int, _path2: CStr) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sync() -> Result<()> { Err(Errno(ENOSYS)) }
    fn unlinkat(_fd: c_int, _path: CStr, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn mlock(_addr: *const c_void, _len: usize) -> Result<()> { Ok(()) }
    unsafe fn mlockall(_flags: c_int) -> Result<()> { Ok(()) }
    unsafe fn mremap(_addr: *mut c_void, _len: usize, _new_len: usize, _flags: c_int, _args: *mut c_void) -> Result<*mut c_void> { Err(Errno(ENOSYS)) }
    unsafe fn mprotect(_addr: *mut c_void, _len: usize, _prot: c_int) -> Result<()> { Ok(()) }
    unsafe fn msync(_addr: *mut c_void, _len: usize, _flags: c_int) -> Result<()> { Ok(()) }
    unsafe fn munlock(_addr: *const c_void, _len: usize) -> Result<()> { Ok(()) }
    unsafe fn madvise(_addr: *mut c_void, _len: usize, _flags: c_int) -> Result<()> { Ok(()) }
    unsafe fn munlockall() -> Result<()> { Ok(()) }
    unsafe fn nanosleep(rqtp: *const timespec, _rmtp: *mut timespec) -> Result<()> {
        let rq = unsafe { &*rqtp };
        let deadline = now_ms() + rq.tv_sec * 1000 + rq.tv_nsec / 1_000_000;
        while now_ms() < deadline {
            unsafe { lowlevel::syscall0(lowlevel::MK_YIELD); }
        }
        Ok(())
    }
    fn getpagesize() -> usize { 4096 }
    fn clock_getres(_clk_id: clockid_t, tp: Option<Out<timespec>>) -> Result<()> {
        if let Some(mut tp) = tp {
            tp.write(timespec { tv_sec: 0, tv_nsec: 1_000_000 });
        }
        Ok(())
    }
    fn clock_gettime(_clk_id: clockid_t, mut tp: Out<timespec>) -> Result<()> {
        let ms = now_ms();
        tp.write(timespec { tv_sec: ms / 1000, tv_nsec: (ms % 1000) * 1_000_000 });
        Ok(())
    }
    unsafe fn clock_settime(_clk_id: clockid_t, _tp: *const timespec) -> Result<()> { Err(Errno(ENOSYS)) }
    fn gettimeofday(mut tp: Out<timeval>, _tzp: Option<Out<timezone>>) -> Result<()> {
        let ms = now_ms();
        tp.write(timeval { tv_sec: ms / 1000, tv_usec: ((ms % 1000) * 1000) as suseconds_t });
        Ok(())
    }
    fn timer_create(_clock_id: clockid_t, _evp: &sigevent, _timerid: Out<timer_t>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn timer_delete(_timerid: timer_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn timer_gettime(_timerid: timer_t, _value: Out<itimerspec>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn timer_settime(_timerid: timer_t, _flags: c_int, _value: &itimerspec, _ovalue: Option<Out<itimerspec>>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getegid() -> gid_t { 0 }
    fn geteuid() -> uid_t { 0 }
    fn getgid() -> gid_t { 0 }
    fn getgroups(_list: Out<[gid_t]>) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn getpgid(_pid: pid_t) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn getppid() -> pid_t { 0 }
    fn getpriority(_which: c_int, _who: id_t) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn getrandom(buf: &mut [u8], _flags: c_uint) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut filled = 0usize;
        for chunk in buf.chunks_mut(4096) {
            let r = unsafe { lowlevel::syscall2(lowlevel::MK_CRYPTO_RANDOM, chunk.as_mut_ptr() as u64, chunk.len() as u64) };
            if r < 0 {
                return Err(Errno(EIO));
            }
            filled += r as usize;
        }
        Ok(filled)
    }
    fn getresgid(_rgid: Option<Out<gid_t>>, _egid: Option<Out<gid_t>>, _sgid: Option<Out<gid_t>>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getresuid(_ruid: Option<Out<uid_t>>, _euid: Option<Out<uid_t>>, _suid: Option<Out<uid_t>>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getrlimit(_resource: c_int, _rlim: Out<rlimit>) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn setrlimit(_resource: c_int, _rlim: *const rlimit) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getrusage(_who: c_int, _r_usage: Out<rusage>) -> Result<()> { Err(Errno(ENOSYS)) }
    fn getsid(_pid: pid_t) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn gettid() -> pid_t { 0 }
    fn getuid() -> uid_t { 0 }
    unsafe fn setgroups(_size: size_t, _list: *const gid_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn setpgid(_pid: pid_t, _pgid: pid_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn setpriority(_which: c_int, _who: id_t, _prio: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn setresgid(_rgid: gid_t, _egid: gid_t, _sgid: gid_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn setresuid(_ruid: uid_t, _euid: uid_t, _suid: uid_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn setsid() -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn umask(_mask: mode_t) -> mode_t { 0 }
    unsafe fn execve(_path: CStr, _argv: *const *mut c_char, _envp: *const *mut c_char) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn fexecve(_fildes: c_int, _argv: *const *mut c_char, _envp: *const *mut c_char) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn exit_thread(_stack_base: *mut (), _stack_size: usize) -> ! { Self::exit(0) }
    unsafe fn fork() -> Result<pid_t> { Err(Errno(ENOSYS)) }
    unsafe fn futex_wait(_addr: *mut u32, _val: u32, _deadline: Option<&timespec>) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn futex_wake(_addr: *mut u32, _num: u32) -> Result<u32> { Err(Errno(ENOSYS)) }
    unsafe fn rlct_clone(_stack: *mut usize, _os_specific: &mut OsSpecific) -> Result<pthread::OsTid> { Err(Errno(ENOSYS)) }
    unsafe fn rlct_kill(_os_tid: pthread::OsTid, _signal: usize) -> Result<()> { Err(Errno(ENOSYS)) }
    fn current_os_tid() -> pthread::OsTid { pthread::OsTid::default() }
    unsafe fn spawn(_program: CStr, _fac: Option<&crate::header::spawn::posix_spawn_file_actions_t>, _fat: Option<&crate::header::spawn::posix_spawnattr_t>, _argv: NulTerminated<*mut c_char>, _envp: Option<NulTerminated<*mut c_char>>) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn waitpid(_pid: pid_t, _stat_loc: Option<Out<c_int>>, _options: c_int) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn sched_yield() -> Result<()> { Err(Errno(ENOSYS)) }
    fn uname(_utsname: Out<utsname>) -> Result<()> { Err(Errno(ENOSYS)) }

    fn verify() -> bool {
        true
    }
}
