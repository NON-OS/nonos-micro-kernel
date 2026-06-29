use super::{
    super::{PalSocket, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{
        errno::{EAFNOSUPPORT, EBADF, EINVAL, EMFILE, ENOSYS},
        sys_socket::{msghdr, sockaddr, socklen_t},
    },
};

impl PalSocket for Sys {
    unsafe fn accept(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<c_int> { Err(Errno(ENOSYS)) }
    unsafe fn bind(_socket: c_int, _address: *const sockaddr, _len: socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn connect(_socket: c_int, _address: *const sockaddr, _len: socklen_t) -> Result<c_int> { Err(Errno(ENOSYS)) }
    unsafe fn getpeername(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn getsockname(_socket: c_int, _address: *mut sockaddr, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn getsockopt(_socket: c_int, _level: c_int, _name: c_int, _val: *mut c_void, _len: *mut socklen_t) -> Result<()> { Err(Errno(ENOSYS)) }
    fn listen(_socket: c_int, _backlog: c_int) -> Result<()> { Err(Errno(ENOSYS)) }
    unsafe fn recvfrom(_socket: c_int, _buf: *mut c_void, _len: size_t, _flags: c_int, _addr: *mut sockaddr, _alen: *mut socklen_t) -> Result<usize> { Err(Errno(ENOSYS)) }
    unsafe fn recvmsg(_socket: c_int, _msg: *mut msghdr, _flags: c_int) -> Result<usize> { Err(Errno(ENOSYS)) }
    unsafe fn sendmsg(_socket: c_int, _msg: *const msghdr, _flags: c_int) -> Result<usize> { Err(Errno(ENOSYS)) }
    unsafe fn sendto(_socket: c_int, _buf: *const c_void, _len: size_t, _flags: c_int, _addr: *const sockaddr, _alen: socklen_t) -> Result<usize> { Err(Errno(ENOSYS)) }
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
