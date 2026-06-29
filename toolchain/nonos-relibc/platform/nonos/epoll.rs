use super::{
    super::{PalEpoll, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{
        bits_sigset_t::sigset_t,
        errno::{EINVAL, EMFILE, ENOENT, ENOSYS},
        sys_epoll::epoll_event,
    },
};

impl PalEpoll for Sys {
    fn epoll_create1(_flags: c_int) -> Result<c_int> {
        super::epoll_rt::create().ok_or(Errno(EMFILE))
    }
    unsafe fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> Result<()> {
        use super::epoll_rt as rt;
        use crate::header::sys_epoll::{EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD};
        match op {
            EPOLL_CTL_ADD | EPOLL_CTL_MOD => {
                if event.is_null() { return Err(Errno(EINVAL)); }
                let events = unsafe { (*event).events };
                let data = unsafe { (*event).data.u64 };
                if rt::set(epfd, fd, events, data) { Ok(()) } else { Err(Errno(EINVAL)) }
            }
            EPOLL_CTL_DEL => {
                if rt::remove(epfd, fd) { Ok(()) } else { Err(Errno(ENOENT)) }
            }
            _ => Err(Errno(EINVAL)),
        }
    }
    unsafe fn epoll_pwait(_epfd: c_int, _events: *mut epoll_event, _maxevents: c_int, _timeout: c_int, _sigmask: *const sigset_t) -> Result<usize> { Err(Errno(ENOSYS)) }
}
