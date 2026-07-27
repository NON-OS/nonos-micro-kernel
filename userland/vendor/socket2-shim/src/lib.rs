// NONOS shim for socket2 0.6.5.
//
// tokio pulls socket2 unconditionally, but on NONOS every real socket operation
// (connect, bind, accept, read, write) goes through mio, whose NONOS backend
// talks to the net.sockets service. socket2 only backs the optional socket-
// option surface (TcpSocket builder, linger/nodelay/keepalive/buffer sizes) and
// NONOS cannot honor those, so this shim provides exactly the subset tokio
// references, every operation returning `Unsupported`. It exists so unmodified
// tokio compiles for NONOS; it is never on a working program's hot path.
//
// This is a NONOS build shim, not upstream socket2. It is patched in only for
// target_vendor = "nonos"; every other target uses the real crate.

use std::fmt;
use std::io;
use std::net::SocketAddr;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::time::Duration;

fn unsupported<T>() -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "socket2 operations are unsupported on NONOS; use tokio's mio-backed sockets",
    ))
}

/// Communication domain (address family).
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Domain(i32);

impl Domain {
    pub const IPV4: Domain = Domain(2);
    pub const IPV6: Domain = Domain(10);
    pub const UNIX: Domain = Domain(1);

    pub const fn for_address(address: SocketAddr) -> Domain {
        match address {
            SocketAddr::V4(_) => Domain::IPV4,
            SocketAddr::V6(_) => Domain::IPV6,
        }
    }
}

impl From<i32> for Domain {
    fn from(d: i32) -> Domain {
        Domain(d)
    }
}
impl From<Domain> for i32 {
    fn from(d: Domain) -> i32 {
        d.0
    }
}
impl fmt::Debug for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Domain").field("0", &self.0).finish()
    }
}

/// Communication type (stream vs datagram).
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Type(i32);

impl Type {
    pub const STREAM: Type = Type(1);
    pub const DGRAM: Type = Type(2);

    pub const fn nonblocking(self) -> Type {
        self
    }
    pub const fn cloexec(self) -> Type {
        self
    }
}

impl From<i32> for Type {
    fn from(t: i32) -> Type {
        Type(t)
    }
}
impl From<Type> for i32 {
    fn from(t: Type) -> i32 {
        t.0
    }
}
impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Type").field("0", &self.0).finish()
    }
}

/// Protocol.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Protocol(i32);

impl Protocol {
    pub const TCP: Protocol = Protocol(6);
    pub const UDP: Protocol = Protocol(17);
    pub const ICMPV4: Protocol = Protocol(1);
    pub const ICMPV6: Protocol = Protocol(58);
}

impl From<i32> for Protocol {
    fn from(p: i32) -> Protocol {
        Protocol(p)
    }
}
impl From<Protocol> for i32 {
    fn from(p: Protocol) -> i32 {
        p.0
    }
}
impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Protocol").field("0", &self.0).finish()
    }
}

/// Socket address. On NONOS the shim only needs to carry a std `SocketAddr`
/// so the tokio call sites type-check; the socket ops that would consume it
/// are all unsupported.
#[derive(Clone)]
pub struct SockAddr(Option<SocketAddr>);

impl SockAddr {
    pub fn as_socket(&self) -> Option<SocketAddr> {
        self.0
    }
}

impl From<SocketAddr> for SockAddr {
    fn from(addr: SocketAddr) -> SockAddr {
        SockAddr(Some(addr))
    }
}
impl fmt::Debug for SockAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SockAddr").finish()
    }
}

// The socket-option surface tokio calls, shared by `Socket` and `SockRef`.
// Every setter is a no-op-returning-error and every getter is unsupported,
// because NONOS net.sockets does not expose these knobs.
macro_rules! socket_options {
    () => {
        pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn set_nodelay(&self, _nodelay: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn nodelay(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_tcp_nodelay(&self, _nodelay: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn tcp_nodelay(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_reuse_address(&self, _reuse: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn reuse_address(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_reuse_port(&self, _reuse: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn reuse_port(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_send_buffer_size(&self, _size: usize) -> io::Result<()> {
            unsupported()
        }
        pub fn send_buffer_size(&self) -> io::Result<usize> {
            unsupported()
        }
        pub fn set_recv_buffer_size(&self, _size: usize) -> io::Result<()> {
            unsupported()
        }
        pub fn recv_buffer_size(&self) -> io::Result<usize> {
            unsupported()
        }
        pub fn set_keepalive(&self, _keepalive: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn keepalive(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_linger(&self, _linger: Option<Duration>) -> io::Result<()> {
            unsupported()
        }
        pub fn linger(&self) -> io::Result<Option<Duration>> {
            unsupported()
        }
        pub fn set_tcp_quickack(&self, _quickack: bool) -> io::Result<()> {
            unsupported()
        }
        pub fn tcp_quickack(&self) -> io::Result<bool> {
            unsupported()
        }
        pub fn set_tos_v4(&self, _tos: u32) -> io::Result<()> {
            unsupported()
        }
        pub fn tos_v4(&self) -> io::Result<u32> {
            unsupported()
        }
        pub fn set_tclass_v6(&self, _tclass: u32) -> io::Result<()> {
            unsupported()
        }
        pub fn tclass_v6(&self) -> io::Result<u32> {
            unsupported()
        }
        pub fn bind_device(&self, _interface: Option<&[u8]>) -> io::Result<()> {
            unsupported()
        }
        pub fn device(&self) -> io::Result<Option<std::vec::Vec<u8>>> {
            unsupported()
        }
        pub fn take_error(&self) -> io::Result<Option<io::Error>> {
            unsupported()
        }
        pub fn local_addr(&self) -> io::Result<SockAddr> {
            unsupported()
        }
        pub fn peek_sender(&self) -> io::Result<SockAddr> {
            unsupported()
        }
    };
}

/// An owned socket. On NONOS creation is unsupported; the type exists so the
/// tokio `TcpSocket` builder type-checks. It owns an `OwnedFd` purely so the
/// fd-conversion traits are well-formed, but `new` never yields one.
pub struct Socket {
    fd: OwnedFd,
}

impl Socket {
    pub fn new(_domain: Domain, _ty: Type, _protocol: Option<Protocol>) -> io::Result<Socket> {
        unsupported()
    }

    pub fn bind(&self, _address: &SockAddr) -> io::Result<()> {
        unsupported()
    }
    pub fn listen(&self, _backlog: i32) -> io::Result<()> {
        unsupported()
    }
    pub fn connect(&self, _address: &SockAddr) -> io::Result<()> {
        unsupported()
    }

    socket_options!();
}

impl fmt::Debug for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Socket").field("fd", &self.fd.as_raw_fd()).finish()
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}
impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        self.fd.into_raw_fd()
    }
}
impl FromRawFd for Socket {
    unsafe fn from_raw_fd(fd: RawFd) -> Socket {
        // SAFETY: forwarded to the caller's obligation that `fd` is an open,
        // solely-owned descriptor.
        Socket { fd: unsafe { OwnedFd::from_raw_fd(fd) } }
    }
}
impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
impl From<OwnedFd> for Socket {
    fn from(fd: OwnedFd) -> Socket {
        Socket { fd }
    }
}
impl From<Socket> for OwnedFd {
    fn from(socket: Socket) -> OwnedFd {
        socket.fd
    }
}

/// A borrowed reference to a socket, built from any fd-bearing value. tokio
/// uses it to reach the option methods on its own socket types.
pub struct SockRef<'a> {
    _borrow: BorrowedFd<'a>,
}

impl SockRef<'_> {
    socket_options!();
}

impl<'a, T: AsFd> From<&'a T> for SockRef<'a> {
    fn from(value: &'a T) -> SockRef<'a> {
        SockRef { _borrow: value.as_fd() }
    }
}

impl fmt::Debug for SockRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SockRef").finish()
    }
}
