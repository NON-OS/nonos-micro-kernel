// net.sockets IPC for the mio backend: resolve the service port, frame a
// request, and read the reply. The wire format mirrors the NONOS std net
// transport (magic, version, op, reserved, body-length header, then body), so
// a socket this backend opens is byte-identical to one the PAL opens and the
// two share the os::fd table. The tcp and udp modules build their operations on
// `sk`; the selector reads readiness with `poll_handle`.

use super::syscall::{sys5, sys6, tag4};

const SK_NAME: &[u8] = b"net.sockets";
const SK_MAGIC: u32 = 0x4E53_4B54;
const OP_POLL: u16 = 13;

// 20-byte request/reply header: magic, version, op, two reserved u32s, body
// length. The reply carries its status at byte 8 and its body at BODY.
pub(super) const BODY: usize = 20;
const TIMEOUT_MS: u64 = 2000;

pub(super) const POLL_READABLE: u8 = 0x01;
pub(super) const POLL_WRITABLE: u8 = 0x02;

// Resolve the net.sockets service port by name (MSVL), or None if unavailable.
fn resolve_port() -> Option<u32> {
    let mut port = 0u32;
    let mut owner = 0u32;
    // SAFETY: MSVL reads the name buffer and writes the two u32 out-params,
    // all valid for the call.
    let rc = unsafe {
        sys5(
            tag4(b"MSVL"),
            SK_NAME.as_ptr() as u64,
            SK_NAME.len() as u64,
            &mut port as *mut u32 as u64,
            &mut owner as *mut u32 as u64,
            0,
        )
    };
    if rc == 0 { Some(port) } else { None }
}

// One request/reply to net.sockets. Returns the reply bytes on success (status
// zero and at least a full header), or None on any transport or protocol
// failure. `reply_cap` is the extra body capacity to reserve after the header.
pub(super) fn sk(op: u16, body: &[u8], reply_cap: usize) -> Option<Vec<u8>> {
    sk_timed(op, body, reply_cap, TIMEOUT_MS)
}

// As `sk`, but with a caller-chosen deadline. The blocking ops (OP_CONNECT and
// OP_ACCEPT) hold the service reply until the TCP handshake resolves, so they
// need a deadline at least as long as net.sockets' own establish wait; the
// default is fine for the non-blocking ops.
pub(super) fn sk_timed(op: u16, body: &[u8], reply_cap: usize, timeout_ms: u64) -> Option<Vec<u8>> {
    let port = resolve_port()?;

    let mut tx = Vec::with_capacity(BODY + body.len());
    tx.extend_from_slice(&SK_MAGIC.to_le_bytes());
    tx.extend_from_slice(&1u16.to_le_bytes());
    tx.extend_from_slice(&op.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&(body.len() as u32).to_le_bytes());
    tx.extend_from_slice(body);

    let mut rx = vec![0u8; BODY + reply_cap];
    // SAFETY: MICL reads tx[0..tx.len()] and writes rx[0..rx.len()]; both slices
    // are valid and outlive the call.
    let n = unsafe {
        sys6(
            tag4(b"MICL"),
            port as u64,
            tx.as_ptr() as u64,
            tx.len() as u64,
            rx.as_mut_ptr() as u64,
            rx.len() as u64,
            timeout_ms,
        )
    };
    if n < BODY as i64 {
        return None;
    }
    if u16::from_le_bytes([rx[8], rx[9]]) != 0 {
        return None;
    }
    rx.truncate(n as usize);
    Some(rx)
}

// Read a little-endian u32 from `buf` at `off`, or 0 if the reply is short.
pub(super) fn read_u32(buf: &[u8], off: usize) -> u32 {
    match buf.get(off..off + 4) {
        Some(b) => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        None => 0,
    }
}

// The 10-byte endpoint body OP_BIND/OP_CONNECT take: handle, IPv4, port.
pub(super) fn endpoint(handle: u32, ip: [u8; 4], port: u16) -> [u8; 10] {
    let mut b = [0u8; 10];
    b[0..4].copy_from_slice(&handle.to_le_bytes());
    b[4..8].copy_from_slice(&ip);
    b[8..10].copy_from_slice(&port.to_le_bytes());
    b
}

// One OP_POLL round trip for `handle`, returning its readiness bits. A failed
// or short reply reports no readiness, which the selector treats as "not ready
// this round".
pub(super) fn poll_handle(handle: u32) -> u8 {
    match sk(OP_POLL, &handle.to_le_bytes(), 1) {
        Some(rx) => rx.get(BODY).copied().unwrap_or(0),
        None => 0,
    }
}
