use super::{
    super::{PalSocket, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{
        errno::{EAFNOSUPPORT, EBADF, EINPROGRESS, EINVAL, EMFILE, ENOSYS},
        fcntl::{F_GETFL, F_SETFL, O_NONBLOCK, O_RDWR},
        sys_socket::{msghdr, sockaddr, socklen_t},
    },
};

impl PalSocket for Sys {
    unsafe fn accept(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<c_int> { Err(Errno(ENOSYS)) }
    unsafe fn bind(_socket: c_int, _address: *const sockaddr, _len: socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> Result<c_int> {
        use super::socket_rt as rt;
        if address.is_null() || (address_len as usize) < 16 {
            return Err(Errno(EINVAL));
        }
        let sa = address as *const u8;
        let family = u16::from_ne_bytes([unsafe { *sa }, unsafe { *sa.add(1) }]);
        if family != 2 {
            return Err(Errno(EAFNOSUPPORT));
        }
        let port = u16::from_be_bytes([unsafe { *sa.add(2) }, unsafe { *sa.add(3) }]);
        let ip = [unsafe { *sa.add(4) }, unsafe { *sa.add(5) }, unsafe { *sa.add(6) }, unsafe { *sa.add(7) }];
        let handle = rt::handle_of(socket).ok_or(Errno(EBADF))?;
        let mut payload = [0u8; 10];
        payload[0..4].copy_from_slice(&handle.to_le_bytes());
        payload[4..8].copy_from_slice(&ip);
        payload[8..10].copy_from_slice(&port.to_le_bytes());
        let mut resp = [0u8; 24];
        let (errno, _) = rt::nskt_call(rt::OP_CONNECT, &payload, &mut resp)?;
        if errno != 0 {
            return Err(Errno(rt::map_errno(errno)));
        }
        if rt::nonblock_of(socket) {
            return Err(Errno(EINPROGRESS));
        }
        Ok(0)
    }
    unsafe fn getpeername(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn getsockname(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn getsockopt(_socket: c_int, _level: c_int, _name: c_int, _val: *mut c_void, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn listen(_socket: c_int, _backlog: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn recvfrom(socket: c_int, buf: *mut c_void, len: size_t, _flags: c_int, _addr: *mut sockaddr, _alen: *mut socklen_t) -> Result<usize> {
        let slice = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, len) };
        sock_recv(socket, slice)
    }
    unsafe fn recvmsg(_socket: c_int, _msg: *mut msghdr, _flags: c_int) -> Result<usize> { Err(Errno(ENOSYS)) }
    unsafe fn sendmsg(_socket: c_int, _msg: *const msghdr, _flags: c_int) -> Result<usize> { Err(Errno(ENOSYS)) }
    unsafe fn sendto(socket: c_int, buf: *const c_void, len: size_t, _flags: c_int, _addr: *const sockaddr, _alen: socklen_t) -> Result<usize> {
        let slice = unsafe { core::slice::from_raw_parts(buf as *const u8, len) };
        sock_send(socket, slice)
    }
    unsafe fn setsockopt(_socket: c_int, _level: c_int, _name: c_int, _val: *const c_void, _len: socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn shutdown(_socket: c_int, _how: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn socket(domain: c_int, kind: c_int, _protocol: c_int) -> Result<c_int> {
        use super::socket_rt as rt;
        if domain != 2 { return Err(Errno(EAFNOSUPPORT)); }
        let wire_kind = match kind {
            1 => rt::KIND_STREAM,
            2 => rt::KIND_DGRAM,
            _ => return Err(Errno(EINVAL)),
        };
        let mut payload = [0u8; 4];
        payload[0..2].copy_from_slice(&rt::FAMILY_INET.to_le_bytes());
        payload[2..4].copy_from_slice(&wire_kind.to_le_bytes());
        let mut resp = [0u8; 24];
        let (errno, n) = rt::nskt_call(rt::OP_SOCKET, &payload, &mut resp)?;
        if errno != 0 || n < 4 { return Err(Errno(rt::map_errno(errno))); }
        let handle = u32::from_le_bytes([resp[20], resp[21], resp[22], resp[23]]);
        rt::alloc(handle).ok_or(Errno(EMFILE))
    }
    fn socketpair(_domain: c_int, _kind: c_int, _protocol: c_int, _sv: &mut [c_int; 2]) -> Result<()> { Err(Errno(ENOSYS)) }
}

pub fn sock_send(fd: c_int, buf: &[u8]) -> Result<usize> {
    use super::socket_rt as rt;
    let handle = rt::handle_of(fd).ok_or(Errno(EBADF))?;
    let n = core::cmp::min(buf.len(), 65536);
    let mut payload = alloc::vec![0u8; 4 + n];
    payload[0..4].copy_from_slice(&handle.to_le_bytes());
    payload[4..].copy_from_slice(&buf[..n]);
    let mut resp = [0u8; 24];
    let (errno, _) = rt::nskt_call(rt::OP_SEND, &payload, &mut resp)?;
    if errno != 0 { return Err(Errno(rt::map_errno(errno))); }
    Ok(n)
}

pub fn sock_recv(fd: c_int, buf: &mut [u8]) -> Result<usize> {
    use super::socket_rt as rt;
    let handle = rt::handle_of(fd).ok_or(Errno(EBADF))?;
    let payload = handle.to_le_bytes();
    let mut resp = alloc::vec![0u8; 24 + buf.len()];
    let (errno, n) = rt::nskt_call(rt::OP_RECV, &payload, &mut resp)?;
    if errno != 0 { return Err(Errno(rt::map_errno(errno))); }
    let n = core::cmp::min(n, buf.len());
    buf[..n].copy_from_slice(&resp[20..20 + n]);
    Ok(n)
}

pub fn close_fd(fd: c_int) -> Result<()> {
    use super::socket_rt as rt;
    let handle = rt::handle_of(fd).ok_or(Errno(EBADF))?;
    let mut payload = [0u8; 4];
    payload[0..4].copy_from_slice(&handle.to_le_bytes());
    let mut resp = [0u8; 24];
    let (errno, _) = rt::nskt_call(rt::OP_CLOSE, &payload, &mut resp)?;
    rt::free(fd);
    if errno != 0 { return Err(Errno(rt::map_errno(errno))); }
    Ok(())
}

pub fn fcntl_sock(fd: c_int, cmd: c_int, arg: c_ulonglong) -> Result<c_int> {
    use super::socket_rt as rt;
    match cmd {
        F_GETFL => {
            let mut fl = O_RDWR;
            if rt::nonblock_of(fd) { fl |= O_NONBLOCK; }
            Ok(fl)
        }
        F_SETFL => {
            let on = (arg as c_int & O_NONBLOCK) != 0;
            let handle = rt::handle_of(fd).ok_or(Errno(EBADF))?;
            rt::set_nonblock(fd, on);
            let mut payload = [0u8; 8];
            payload[0..4].copy_from_slice(&handle.to_le_bytes());
            payload[4..8].copy_from_slice(&(if on { 1u32 } else { 0u32 }).to_le_bytes());
            let mut resp = [0u8; 24];
            let (errno, _) = rt::nskt_call(rt::OP_SETFLAGS, &payload, &mut resp)?;
            if errno != 0 { return Err(Errno(rt::map_errno(errno))); }
            Ok(0)
        }
        _ => Err(Errno(ENOSYS)),
    }
}
