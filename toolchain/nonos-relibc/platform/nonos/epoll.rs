use super::{
    super::{PalEpoll, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{bits_sigset_t::sigset_t, errno::ENOSYS, sys_epoll::epoll_event},
};

impl PalEpoll for Sys {
    fn epoll_create1(_flags: c_int) -> Result<c_int> { Err(Errno(ENOSYS)) }
    unsafe fn epoll_ctl(_epfd: c_int, _op: c_int, _fd: c_int, _event: *mut epoll_event) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn epoll_pwait(_epfd: c_int, _events: *mut epoll_event, _maxevents: c_int, _timeout: c_int, _sigmask: *const sigset_t) -> Result<usize> { Err(Errno(ENOSYS)) }
}
