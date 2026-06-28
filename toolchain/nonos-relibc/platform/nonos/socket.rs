use super::{
    super::{PalSocket, types::*},
    Sys,
};
use crate::{
    error::{Errno, Result},
    header::{
        errno::ENOSYS,
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
    unsafe fn socket(_domain: c_int, _kind: c_int, _protocol: c_int) -> Result<c_int> { Err(Errno(ENOSYS)) }
    fn socketpair(_domain: c_int, _kind: c_int, _protocol: c_int, _sv: &mut [c_int; 2]) -> Result<()> { Err(Errno(ENOSYS)) }
}
