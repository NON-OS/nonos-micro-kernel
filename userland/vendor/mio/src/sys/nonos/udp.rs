// UDP socket operations for the NONOS backend, over net.sockets. bind opens a
// datagram socket, binds it, and registers it in the os::fd table so the std
// `UdpSocket` mio wraps sends and receives through the same handle.

use std::io;
use std::net::{self, SocketAddr};
use std::os::fd::FromRawFd;

use super::net::{endpoint, sk};

const OP_SOCKET: u16 = 2;
const OP_BIND: u16 = 3;
const KIND_DGRAM: u16 = 2;

fn other(msg: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

fn v4_parts(addr: SocketAddr) -> io::Result<([u8; 4], u16)> {
    match addr {
        SocketAddr::V4(a) => Ok((a.ip().octets(), a.port())),
        SocketAddr::V6(_) => Err(other("net.sockets is IPv4 only")),
    }
}

fn open_socket(kind: u16) -> io::Result<u32> {
    let mut body = [0u8; 4];
    body[0..2].copy_from_slice(&4u16.to_le_bytes());
    body[2..4].copy_from_slice(&kind.to_le_bytes());
    let rx = sk(OP_SOCKET, &body, 8).ok_or_else(|| other("net.sockets OP_SOCKET failed"))?;
    Ok(super::net::read_u32(&rx, super::net::BODY))
}

pub fn bind(addr: SocketAddr) -> io::Result<net::UdpSocket> {
    let (ip, port) = v4_parts(addr)?;
    let handle = open_socket(KIND_DGRAM)?;
    sk(OP_BIND, &endpoint(handle, ip, port), 0).ok_or_else(|| other("net.sockets OP_BIND failed"))?;
    let fd = std::os::fd::nonos_register_socket(handle)?;
    // SAFETY: `fd` was just installed for `handle` and is owned by no other
    // value, so the returned socket takes sole ownership.
    Ok(unsafe { net::UdpSocket::from_raw_fd(fd) })
}

pub(crate) fn only_v6(_socket: &net::UdpSocket) -> io::Result<bool> {
    // NONOS net.sockets is IPv4 only, so a socket is never v6-only.
    Ok(false)
}
