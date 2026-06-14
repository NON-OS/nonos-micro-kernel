// NONOS std net: TCP/UDP over the net.sockets capsule and DNS over the
// net.dns capsule, via IPC. Core connect/read/write, bind/accept, and
// send/recv are wired; socket options are best-effort no-ops, and IPv6
// and multicast are unsupported (the userland stack is IPv4).

use crate::fmt;
use crate::io::{self, BorrowedCursor, Error, ErrorKind, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4, ToSocketAddrs};
use crate::sys::unsupported;
use crate::time::Duration;
use crate::vec::Vec;

const SK_NAME: &[u8] = b"net.sockets";
const SK_MAGIC: u32 = 0x4E53_4B54;
const DNS_NAME: &[u8] = b"net.dns";
const DNS_MAGIC: u32 = 0x4E44_4E53;
const BODY: usize = 20;
const MAX_PAYLOAD: usize = 1536;
const OP_SOCKET: u16 = 2;
const OP_BIND: u16 = 3;
const OP_LISTEN: u16 = 4;
const OP_ACCEPT: u16 = 5;
const OP_CONNECT: u16 = 6;
const OP_SEND: u16 = 7;
const OP_RECV: u16 = 8;
const OP_CLOSE: u16 = 9;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

unsafe fn sys5(num: i64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64) -> i64 {
    let r: i64;
    unsafe {
        core::arch::asm!("syscall", inout("rax") num => r, in("rdi") a1, in("rsi") a2,
            in("rdx") a3, in("r10") a4, in("r8") a5, out("rcx") _, out("r11") _);
    }
    r
}

fn err(msg: &'static str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

fn read_u32(b: &[u8], off: usize) -> u32 {
    if b.len() < off + 4 {
        return 0;
    }
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn ipc(service: &[u8], magic: u32, op: u16, body: &[u8], reply_cap: usize) -> io::Result<Vec<u8>> {
    let mut port = 0u32;
    let mut owner = 0u32;
    let rc = unsafe {
        sys5(tag4(b"MSVL"), service.as_ptr() as u64, service.len() as u64,
            &mut port as *mut u32 as u64, &mut owner as *mut u32 as u64, 0)
    };
    if rc != 0 {
        return Err(err("net service unavailable"));
    }
    let mut tx = Vec::with_capacity(BODY + body.len());
    tx.extend_from_slice(&magic.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);
    let mut rx = crate::vec![0u8; BODY + reply_cap];
    let n = unsafe {
        sys5(tag4(b"MICL"), port as u64, tx.as_ptr() as u64, tx.len() as u64,
            rx.as_mut_ptr() as u64, rx.len() as u64)
    };
    if n < BODY as i64 {
        return Err(err("net ipc failed"));
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return Err(err("net op rejected"));
    }
    rx.truncate(n as usize);
    Ok(rx)
}

fn sk(op: u16, body: &[u8], cap: usize) -> io::Result<Vec<u8>> {
    ipc(SK_NAME, SK_MAGIC, op, body, cap)
}

fn v4_parts(addr: &SocketAddr) -> io::Result<([u8; 4], u16)> {
    match addr {
        SocketAddr::V4(a) => Ok((a.ip().octets(), a.port())),
        SocketAddr::V6(_) => Err(err("ipv6 unsupported")),
    }
}

fn endpoint(handle: u32, ip: [u8; 4], port: u16) -> [u8; 10] {
    let mut b = [0u8; 10];
    b[0..4].copy_from_slice(&handle.to_le_bytes());
    b[4..8].copy_from_slice(&ip);
    b[8..10].copy_from_slice(&port.to_le_bytes());
    b
}

fn open_socket(kind: u16) -> io::Result<u32> {
    let mut body = [0u8; 4];
    body[0..2].copy_from_slice(&4u16.to_le_bytes());
    body[2..4].copy_from_slice(&kind.to_le_bytes());
    Ok(read_u32(&sk(OP_SOCKET, &body, 8)?, BODY))
}

fn send_on(handle: u32, buf: &[u8]) -> io::Result<usize> {
    let n = buf.len().min(MAX_PAYLOAD);
    let mut body = Vec::with_capacity(4 + n);
    body.extend_from_slice(&handle.to_le_bytes());
    body.extend_from_slice(&buf[..n]);
    sk(OP_SEND, &body, 0)?;
    Ok(n)
}

fn recv_on(handle: u32, buf: &mut [u8]) -> io::Result<usize> {
    let rx = sk(OP_RECV, &handle.to_le_bytes(), MAX_PAYLOAD)?;
    let data = &rx[BODY..];
    let n = data.len().min(buf.len());
    buf[..n].copy_from_slice(&data[..n]);
    Ok(n)
}

fn close(handle: u32) {
    let _ = sk(OP_CLOSE, &handle.to_le_bytes(), 0);
}

fn unspecified() -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0))
}

pub struct TcpStream {
    handle: u32,
    peer: SocketAddr,
}

impl TcpStream {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            match Self::connect_addr(&a) {
                Ok(s) => return Ok(s),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    fn connect_addr(addr: &SocketAddr) -> io::Result<TcpStream> {
        let (ip, port) = v4_parts(addr)?;
        let handle = open_socket(1)?;
        if let Err(e) = sk(OP_CONNECT, &endpoint(handle, ip, port), 0) {
            close(handle);
            return Err(e);
        }
        Ok(TcpStream { handle, peer: *addr })
    }

    pub fn connect_timeout(addr: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        Self::connect_addr(addr)
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        recv_on(self.handle, buf)
    }
    pub fn read_buf(&self, mut cursor: BorrowedCursor<'_>) -> io::Result<()> {
        let mut tmp = [0u8; 1536];
        let n = self.read(&mut tmp)?;
        cursor.append(&tmp[..n]);
        Ok(())
    }
    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.read(b);
            }
        }
        Ok(0)
    }
    pub fn is_read_vectored(&self) -> bool {
        false
    }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        send_on(self.handle, buf)
    }
    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        for b in bufs {
            if !b.is_empty() {
                return self.write(b);
            }
        }
        Ok(0)
    }
    pub fn is_write_vectored(&self) -> bool {
        false
    }
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.peer)
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(unspecified())
    }
    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        Ok(())
    }
    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unsupported()
    }
    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn linger(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn nodelay(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        close(self.handle);
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream").field("peer", &self.peer).finish()
    }
}

pub struct TcpListener {
    handle: u32,
    local: SocketAddr,
}

impl TcpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            match Self::bind_addr(&a) {
                Ok(l) => return Ok(l),
                Err(e) => last = e,
            }
        }
        Err(last)
    }

    fn bind_addr(addr: &SocketAddr) -> io::Result<TcpListener> {
        let (ip, port) = v4_parts(addr)?;
        let handle = open_socket(1)?;
        if let Err(e) = sk(OP_BIND, &endpoint(handle, ip, port), 0) {
            close(handle);
            return Err(e);
        }
        if let Err(e) = sk(OP_LISTEN, &handle.to_le_bytes(), 0) {
            close(handle);
            return Err(e);
        }
        Ok(TcpListener { handle, local: *addr })
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let child = read_u32(&sk(OP_ACCEPT, &self.handle.to_le_bytes(), 8)?, BODY);
        Ok((TcpStream { handle: child, peer: unspecified() }, unspecified()))
    }
    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unsupported()
    }
    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn only_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
}

impl Drop for TcpListener {
    fn drop(&mut self) {
        close(self.handle);
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpListener").field("local", &self.local).finish()
    }
}

pub struct UdpSocket {
    handle: u32,
    local: SocketAddr,
}

impl UdpSocket {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            let (ip, port) = match v4_parts(&a) {
                Ok(v) => v,
                Err(e) => {
                    last = e;
                    continue;
                }
            };
            let handle = open_socket(2)?;
            match sk(OP_BIND, &endpoint(handle, ip, port), 0) {
                Ok(_) => return Ok(UdpSocket { handle, local: a }),
                Err(e) => {
                    close(handle);
                    last = e;
                }
            }
        }
        Err(last)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(unspecified())
    }
    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }
    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        let n = recv_on(self.handle, buf)?;
        Ok((n, unspecified()))
    }
    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }
    pub fn send_to(&self, buf: &[u8], addr: &SocketAddr) -> io::Result<usize> {
        let (ip, port) = v4_parts(addr)?;
        sk(OP_CONNECT, &endpoint(self.handle, ip, port), 0)?;
        send_on(self.handle, buf)
    }
    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unsupported()
    }
    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        Ok(None)
    }
    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn broadcast(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        Ok(1)
    }
    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        Ok(false)
    }
    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }
    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }
    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }
    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }
    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        Ok(())
    }
    pub fn ttl(&self) -> io::Result<u32> {
        Ok(64)
    }
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }
    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        Ok(())
    }
    pub fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        recv_on(self.handle, buf)
    }
    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }
    pub fn send(&self, buf: &[u8]) -> io::Result<usize> {
        send_on(self.handle, buf)
    }
    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        let mut last = err("no address");
        for a in addr.to_socket_addrs()? {
            let (ip, port) = match v4_parts(&a) {
                Ok(v) => v,
                Err(e) => {
                    last = e;
                    continue;
                }
            };
            match sk(OP_CONNECT, &endpoint(self.handle, ip, port), 0) {
                Ok(_) => return Ok(()),
                Err(e) => last = e,
            }
        }
        Err(last)
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        close(self.handle);
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpSocket").field("local", &self.local).finish()
    }
}

pub struct LookupHost {
    it: crate::vec::IntoIter<SocketAddr>,
    port: u16,
}

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.it.next()
    }
}

pub fn lookup_host(host: &str, port: u16) -> io::Result<LookupHost> {
    let ip = if let Ok(v4) = host.parse::<Ipv4Addr>() {
        v4
    } else {
        if host.is_empty() || host.len() > 255 {
            return Err(err("bad hostname"));
        }
        let rx = ipc(DNS_NAME, DNS_MAGIC, 2, host.as_bytes(), 4)?;
        let b = &rx[BODY..];
        if b.len() < 4 {
            return Err(err("dns short reply"));
        }
        Ipv4Addr::new(b[0], b[1], b[2], b[3])
    };
    let mut v: Vec<SocketAddr> = Vec::with_capacity(1);
    v.push(SocketAddr::V4(SocketAddrV4::new(ip, port)));
    Ok(LookupHost { it: v.into_iter(), port })
}
