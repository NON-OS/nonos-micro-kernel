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
        fcntl::{O_APPEND, O_CREAT, O_TRUNC},
        signal::sigevent,
        sys_resource::{rlimit, rusage},
        sys_select::timeval,
        sys_stat::{stat, S_IFDIR, S_IFREG},
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
        let (status, data_len) = fs::vfs_call(fs::OP_WRITE, &payload, &mut resp)?;
        if data_len < 4 { return Err(Errno(EIO)); }
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

    fn faccessat(_fd: c_int, path: CStr, _amode: c_int, _flags: c_int) -> Result<()> {
        let pb = path.to_bytes();
        if pb.is_empty() || pb.len() > 255 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = Vec::with_capacity(5 + pb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(pb.len() as u8);
        payload.extend_from_slice(pb);
        let mut resp = [0u8; 36];
        let (status, _) = fs::vfs_call(fs::OP_STAT, &payload, &mut resp)?;
        if status < 0 { Err(Errno(-status)) } else { Ok(()) }
    }
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
        if fs::fd_vfs(fildes2).is_some() { let _ = Self::close(fildes2); }
        if !fs::fd_set(fildes2, vfs_fd) { return Err(Errno(EBADF)); }
        Ok(fildes2)
    }
    fn fchdir(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fchmodat(_dirfd: c_int, _path: Option<CStr>, _mode: mode_t, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fchownat(_fildes: c_int, _path: CStr, _owner: uid_t, _group: gid_t, _flags: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fdatasync(_fildes: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn flock(_fd: c_int, _operation: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    fn fstatat(_fildes: c_int, path: Option<CStr>, mut buf: Out<stat>, _flags: c_int) -> Result<()> {
        let p = match path {
            Some(p) if !p.to_bytes().is_empty() => p,
            _ => return Err(Errno(ENOSYS)),
        };
        let pb = p.to_bytes();
        if pb.len() > 255 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = Vec::with_capacity(5 + pb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(pb.len() as u8);
        payload.extend_from_slice(pb);
        let mut resp = [0u8; 36];
        let (status, data_len) = fs::vfs_call(fs::OP_STAT, &payload, &mut resp)?;
        if data_len < 12 { return Err(Errno(EIO)); }
        if status < 0 { return Err(Errno(-status)); }
        let size = u64::from_le_bytes([resp[24], resp[25], resp[26], resp[27],
                                       resp[28], resp[29], resp[30], resp[31]]);
        let vfs_flags = u32::from_le_bytes([resp[32], resp[33], resp[34], resp[35]]);
        buf.write(stat {
            st_size: size as off_t,
            st_mode: if vfs_flags & 1 != 0 { S_IFDIR | 0o555 } else { S_IFREG | 0o644 },
            ..Default::default()
        });
        Ok(())
    }
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
    fn lseek(fildes: c_int, offset: off_t, whence: c_int) -> Result<off_t> {
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        let pid = Self::getpid() as u32;
        let mut payload = [0u8; 20];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        payload[8..10].copy_from_slice(&(whence as u16).to_le_bytes());
        payload[12..20].copy_from_slice(&offset.to_le_bytes());
        let mut resp = [0u8; 32];
        let (status, data_len) = fs::vfs_call(fs::OP_SEEK, &payload, &mut resp)?;
        if status < 0 { return Err(Errno(-status)); }
        if data_len < 8 { return Err(Errno(EIO)); }
        Ok(i64::from_le_bytes([resp[24], resp[25], resp[26], resp[27], resp[28], resp[29], resp[30], resp[31]]) as off_t)
    }
    fn mkdirat(_fildes: c_int, path: CStr, _mode: mode_t) -> Result<()> {
        let pb = path.to_bytes();
        if pb.is_empty() || pb.len() > 255 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = Vec::with_capacity(5 + pb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(pb.len() as u8);
        payload.extend_from_slice(pb);
        let mut resp = [0u8; 24];
        let (status, _) = fs::vfs_call(fs::OP_MKDIR, &payload, &mut resp)?;
        if status < 0 { Err(Errno(-status)) } else { Ok(()) }
    }
    fn mkfifoat(_dir_fd: c_int, _path: CStr, _mode: mode_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn mknodat(_fildes: c_int, _path: CStr, _mode: mode_t, _dev: dev_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn openat(_dirfd: c_int, path: CStr, oflag: c_int, _mode: mode_t) -> Result<c_int> {
        let pb = path.to_bytes();
        if pb.is_empty() || pb.len() > 255 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let vfs_flags =
            (if oflag & O_CREAT != 0 { fs::VFS_O_CREATE } else { 0 }) |
            (if oflag & O_TRUNC != 0 { fs::VFS_O_TRUNC } else { 0 }) |
            (if oflag & O_APPEND != 0 { fs::VFS_O_APPEND } else { 0 });
        let mut payload = Vec::with_capacity(9 + pb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(pb.len() as u8);
        payload.extend_from_slice(pb);
        payload.extend_from_slice(&vfs_flags.to_le_bytes());
        let mut resp = [0u8; 28];
        let (status, data_len) = fs::vfs_call(fs::OP_OPEN, &payload, &mut resp)?;
        if data_len < 4 { return Err(Errno(EIO)); }
        if status < 0 { return Err(Errno(-status)); }
        let vfs_fd = u32::from_le_bytes([resp[24], resp[25], resp[26], resp[27]]);
        fs::fd_alloc(vfs_fd).ok_or(Errno(EMFILE))
    }
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
    fn pread(fildes: c_int, buf: &mut [u8], offset: off_t) -> Result<usize> {
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        let count = (buf.len() as u32).min(65536);
        let pid = Self::getpid() as u32;
        let mut payload = [0u8; 20];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        payload[8..16].copy_from_slice(&(offset as i64).to_le_bytes());
        payload[16..20].copy_from_slice(&count.to_le_bytes());
        let mut resp = alloc::vec![0u8; 24 + count as usize];
        let (status, n) = fs::vfs_call(fs::OP_PREAD, &payload, &mut resp)?;
        if status < 0 { return Err(Errno(-status)); }
        buf[..n].copy_from_slice(&resp[24..24 + n]);
        Ok(n)
    }
    fn pwrite(fildes: c_int, buf: &[u8], offset: off_t) -> Result<usize> {
        let vfs_fd = fs::fd_vfs(fildes).ok_or(Errno(EBADF))?;
        if buf.len() > 65536 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = alloc::vec![0u8; 16 + buf.len()];
        payload[0..4].copy_from_slice(&pid.to_le_bytes());
        payload[4..8].copy_from_slice(&vfs_fd.to_le_bytes());
        payload[8..16].copy_from_slice(&(offset as i64).to_le_bytes());
        payload[16..].copy_from_slice(buf);
        let mut resp = [0u8; 28];
        let (status, data_len) = fs::vfs_call(fs::OP_PWRITE, &payload, &mut resp)?;
        if status < 0 { return Err(Errno(-status)); }
        if data_len < 4 { return Err(Errno(EIO)); }
        Ok(u32::from_le_bytes([resp[24], resp[25], resp[26], resp[27]]) as usize)
    }
    fn readlinkat(_dirfd: c_int, _pathname: CStr, _out: &mut [u8]) -> Result<usize> { Err(Errno(ENOSYS)) }
    fn renameat2(_old_dir: c_int, old_path: CStr, _new_dir: c_int, new_path: CStr, _flags: c_uint) -> Result<()> {
        let ob = old_path.to_bytes();
        let nb = new_path.to_bytes();
        if ob.is_empty() || ob.len() > 255 || nb.is_empty() || nb.len() > 255 {
            return Err(Errno(EINVAL));
        }
        let pid = Self::getpid() as u32;
        let mut payload = Vec::with_capacity(4 + 1 + ob.len() + 1 + nb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(ob.len() as u8);
        payload.extend_from_slice(ob);
        payload.push(nb.len() as u8);
        payload.extend_from_slice(nb);
        let mut resp = [0u8; 24];
        let (status, _) = fs::vfs_call(fs::OP_RENAME, &payload, &mut resp)?;
        if status < 0 { Err(Errno(-status)) } else { Ok(()) }
    }
    fn symlinkat(_path1: CStr, _fd: c_int, _path2: CStr) -> Result<()> { Err(Errno(ENOSYS)) }
    fn sync() -> Result<()> { Err(Errno(ENOSYS)) }
    fn unlinkat(_fd: c_int, path: CStr, _flags: c_int) -> Result<()> {
        let pb = path.to_bytes();
        if pb.is_empty() || pb.len() > 255 { return Err(Errno(EINVAL)); }
        let pid = Self::getpid() as u32;
        let mut payload = Vec::with_capacity(5 + pb.len());
        payload.extend_from_slice(&pid.to_le_bytes());
        payload.push(pb.len() as u8);
        payload.extend_from_slice(pb);
        let mut resp = [0u8; 24];
        let (status, _) = fs::vfs_call(fs::OP_UNLINK, &payload, &mut resp)?;
        if status < 0 { Err(Errno(-status)) } else { Ok(()) }
    }
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
    unsafe fn exit_thread(_stack_base: *mut (), _stack_size: usize) -> ! {
        unsafe { lowlevel::syscall1(lowlevel::MK_EXIT, 0); }
        loop {}
    }
    unsafe fn fork() -> Result<pid_t> { Err(Errno(ENOSYS)) }
    unsafe fn futex_wait(_addr: *mut u32, _val: u32, _deadline: Option<&timespec>) -> Result<()> { Ok(()) }
    unsafe fn futex_wake(_addr: *mut u32, _num: u32) -> Result<u32> { Ok(0) }
    unsafe fn rlct_clone(stack: *mut usize, _os_specific: &mut OsSpecific) -> Result<pthread::OsTid> {
        let r = unsafe { lowlevel::syscall2(lowlevel::MK_THREAD_SPAWN, lowlevel::rlct_trampoline as *const () as u64, stack as u64) };
        if r < 0 { return Err(Errno(-r as c_int)); }
        Ok(pthread::OsTid {})
    }
    unsafe fn rlct_kill(_os_tid: pthread::OsTid, _signal: usize) -> Result<()> { Err(Errno(ENOSYS)) }
    fn current_os_tid() -> pthread::OsTid {
        let _pid = unsafe { lowlevel::syscall0(lowlevel::MK_GETPID) };
        pthread::OsTid {}
    }
    unsafe fn spawn(_program: CStr, _fac: Option<&crate::header::spawn::posix_spawn_file_actions_t>, _fat: Option<&crate::header::spawn::posix_spawnattr_t>, _argv: NulTerminated<*mut c_char>, _envp: Option<NulTerminated<*mut c_char>>) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn waitpid(_pid: pid_t, _stat_loc: Option<Out<c_int>>, _options: c_int) -> Result<pid_t> { Err(Errno(ENOSYS)) }
    fn sched_yield() -> Result<()> {
        unsafe { lowlevel::syscall0(lowlevel::MK_YIELD); }
        Ok(())
    }
    fn uname(_utsname: Out<utsname>) -> Result<()> { Err(Errno(ENOSYS)) }

    fn verify() -> bool {
        true
    }
}
