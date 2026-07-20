// TCP socket operations for the NONOS backend, over net.sockets. Each maps a
// mio call onto the service op the PAL uses, resolving a `RawFd` to its
// net.sockets handle through the os::fd table seam. Sockets are created here
// and registered in the table so the std `TcpStream` mio wraps does its read
// and write through the same handle.

use std::io;
use std::net::{self, SocketAddr};
use std::os::fd::{AsRawFd, FromRawFd};

use super::net::{endpoint, read_u32, sk, sk_timed, BODY};

const OP_SOCKET: u16 = 2;
const OP_BIND: u16 = 3;
const OP_LISTEN: u16 = 4;
const OP_ACCEPT: u16 = 5;
// Non-blocking connect: net.sockets initiates the handshake and replies at once;
// tokio confirms the connection by polling writability through the selector.
// A blocking OP_CONNECT would stall the whole current-thread runtime for the
// handshake round trip.
const OP_CONNECT_NB: u16 = 14;
const KIND_STREAM: u16 = 1;

// OP_ACCEPT blocks in net.sockets until a peer arrives, so give its reply room
// rather than tripping the default poll deadline.
const BLOCKING_TIMEOUT_MS: u64 = 10_000;

fn other(msg: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

// NONOS speaks IPv4 to net.sockets; reject v6 rather than silently mistranslate.
fn v4_parts(addr: SocketAddr) -> io::Result<([u8; 4], u16)> {
    match addr {
        SocketAddr::V4(a) => Ok((a.ip().octets(), a.port())),
        SocketAddr::V6(_) => Err(other("net.sockets is IPv4 only")),
    }
}

// Open a net.sockets socket of `kind` and return its handle.
fn open_socket(kind: u16) -> io::Result<u32> {
    let mut body = [0u8; 4];
    body[0..2].copy_from_slice(&4u16.to_le_bytes());
    body[2..4].copy_from_slice(&kind.to_le_bytes());
    let rx = sk(OP_SOCKET, &body, 8).ok_or_else(|| other("net.sockets OP_SOCKET failed"))?;
    Ok(read_u32(&rx, BODY))
}

// The net.sockets handle a live socket descriptor names.
fn handle_of(fd: i32) -> io::Result<u32> {
    std::os::fd::nonos_socket_handle(fd).ok_or_else(|| other("fd is not a live socket"))
}

pub(crate) fn new_for_addr(_addr: SocketAddr) -> io::Result<i32> {
    let handle = open_socket(KIND_STREAM)?;
    std::os::fd::nonos_register_socket(handle)
}

pub(crate) fn connect(socket: &net::TcpStream, addr: SocketAddr) -> io::Result<()> {
    let handle = handle_of(socket.as_raw_fd())?;
    let (ip, port) = v4_parts(addr)?;
    sk(OP_CONNECT_NB, &endpoint(handle, ip, port), 0)
        .ok_or_else(|| other("net.sockets OP_CONNECT failed"))?;
    Ok(())
}

pub(crate) fn bind(socket: &net::TcpListener, addr: SocketAddr) -> io::Result<()> {
    let handle = handle_of(socket.as_raw_fd())?;
    let (ip, port) = v4_parts(addr)?;
    sk(OP_BIND, &endpoint(handle, ip, port), 0).ok_or_else(|| other("net.sockets OP_BIND failed"))?;
    Ok(())
}

pub(crate) fn listen(socket: &net::TcpListener, _backlog: i32) -> io::Result<()> {
    let handle = handle_of(socket.as_raw_fd())?;
    sk(OP_LISTEN, &handle.to_le_bytes(), 0).ok_or_else(|| other("net.sockets OP_LISTEN failed"))?;
    Ok(())
}

pub(crate) fn set_reuseaddr(_socket: &net::TcpListener, _reuseaddr: bool) -> io::Result<()> {
    // net.sockets binds are already exclusive per handle; SO_REUSEADDR has no
    // analog, so accept the request without changing behavior.
    Ok(())
}

pub(crate) fn accept(listener: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
    let handle = handle_of(listener.as_raw_fd())?;
    let rx = sk_timed(OP_ACCEPT, &handle.to_le_bytes(), 8, BLOCKING_TIMEOUT_MS)
        .ok_or_else(|| other("net.sockets OP_ACCEPT failed"))?;
    let child = read_u32(&rx, BODY);
    let fd = std::os::fd::nonos_register_socket(child)?;
    // SAFETY: `fd` was just installed for `child` and is owned by no other
    // value, so handing sole ownership to the returned stream cannot
    // double-close it.
    let stream = unsafe { net::TcpStream::from_raw_fd(fd) };
    // net.sockets does not report the peer address; the unspecified address is
    // the same value the PAL accept path returns.
    Ok((stream, SocketAddr::from(([0, 0, 0, 0], 0))))
}
