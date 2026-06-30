use super::{
    super::{PalEpoll, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{
        bits_sigset_t::sigset_t,
        errno::{EBADF, EINVAL, EMFILE, ENOENT},
        sys_epoll::{epoll_data, epoll_event, EPOLLIN, EPOLLOUT},
    },
};

fn poll_one(handle: u32, events: u16) -> u16 {
    use super::socket_rt as rt;
    let mut payload = [0u8; 12];
    payload[0..4].copy_from_slice(&handle.to_le_bytes());
    payload[4..6].copy_from_slice(&events.to_le_bytes());
    let mut resp = [0u8; 24];
    match rt::nskt_call(rt::OP_POLL, &payload, &mut resp) {
        Ok((0, n)) if n >= 2 => u16::from_le_bytes([resp[20], resp[21]]),
        _ => 0,
    }
}

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
    unsafe fn epoll_pwait(epfd: c_int, events: *mut epoll_event, maxevents: c_int, timeout: c_int, _sigmask: *const sigset_t) -> Result<usize> {
        use super::epoll_rt as ep;
        use super::socket_rt as rt;
        if maxevents <= 0 {
            return Err(Errno(EINVAL));
        }
        let max = maxevents as usize;
        let deadline = super::now_ms().saturating_add(timeout as i64);
        loop {
            let mut n = 0usize;
            let ok = ep::for_each(epfd, |fd, want, data| {
                if n >= max { return; }
                let Some(handle) = rt::handle_of(fd) else { return; };
                let mut pe: u16 = 0;
                if want & EPOLLIN != 0 { pe |= 1; }
                if want & EPOLLOUT != 0 { pe |= 2; }
                let revents = poll_one(handle, pe);
                if revents != 0 {
                    let mut e: u32 = 0;
                    if revents & 1 != 0 { e |= EPOLLIN; }
                    if revents & 2 != 0 { e |= EPOLLOUT; }
                    unsafe {
                        (*events.add(n)).events = e;
                        (*events.add(n)).data = epoll_data { u64: data };
                    }
                    n += 1;
                }
            });
            if !ok { return Err(Errno(EBADF)); }
            if n > 0 || timeout == 0 || (timeout > 0 && super::now_ms() >= deadline) {
                return Ok(n);
            }
            unsafe { super::lowlevel::syscall0(super::lowlevel::MK_YIELD); }
        }
    }
}
