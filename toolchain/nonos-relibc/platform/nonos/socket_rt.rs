use alloc::vec::Vec;
use core::cell::SyncUnsafeCell;

use crate::error::{Errno, Result};
use crate::header::errno::{EAFNOSUPPORT, EAGAIN, EBADF, ECONNREFUSED, EINVAL, EIO, EMFILE, ENOTCONN};
use super::super::types::c_int;
use super::lowlevel::{syscall6, MK_IPC_CALL};

pub const NSKT_ENDPOINT: u64 = 4460;
pub const NSKT_MAGIC: u32 = 0x4E53_4B54;
pub const HDR_LEN: usize = 20;

pub const OP_SOCKET: u16 = 2;
pub const OP_CONNECT: u16 = 6;
pub const OP_SEND: u16 = 7;
pub const OP_RECV: u16 = 8;
pub const OP_CLOSE: u16 = 9;
pub const OP_POLL: u16 = 12;
pub const OP_SETFLAGS: u16 = 13;
pub const OP_SETTIMEOUT: u16 = 15;
pub const E_WOULD_BLOCK: u16 = 11;

pub const FAMILY_INET: u16 = 4;
pub const KIND_STREAM: u16 = 1;
pub const KIND_DGRAM: u16 = 2;

const SOCK_FD_BASE: c_int = 256;
const SOCK_SLOTS: usize = 64;

#[derive(Clone, Copy)]
struct SockEntry { handle: u32, nonblock: bool, used: bool }

static SOCK_TABLE: SyncUnsafeCell<[SockEntry; SOCK_SLOTS]> =
    SyncUnsafeCell::new([SockEntry { handle: 0, nonblock: false, used: false }; SOCK_SLOTS]);

fn table() -> &'static mut [SockEntry; SOCK_SLOTS] { unsafe { &mut *SOCK_TABLE.get() } }

pub fn is_socket_fd(fd: c_int) -> bool {
    fd >= SOCK_FD_BASE && (fd - SOCK_FD_BASE) < SOCK_SLOTS as c_int
}
fn slot(fd: c_int) -> Option<usize> {
    if is_socket_fd(fd) { Some((fd - SOCK_FD_BASE) as usize) } else { None }
}
pub fn handle_of(fd: c_int) -> Option<u32> {
    let e = table()[slot(fd)?];
    if e.used { Some(e.handle) } else { None }
}
pub fn alloc(handle: u32) -> Option<c_int> {
    for (i, e) in table().iter_mut().enumerate() {
        if !e.used {
            *e = SockEntry { handle, nonblock: false, used: true };
            return Some(SOCK_FD_BASE + i as c_int);
        }
    }
    None
}
pub fn free(fd: c_int) {
    if let Some(i) = slot(fd) { table()[i].used = false; }
}
pub fn nonblock_of(fd: c_int) -> bool {
    slot(fd).map(|i| table()[i].nonblock).unwrap_or(false)
}
pub fn set_nonblock(fd: c_int, on: bool) {
    if let Some(i) = slot(fd) { table()[i].nonblock = on; }
}

pub fn map_errno(e: u16) -> c_int {
    match e {
        5 => EBADF,
        6 => ECONNREFUSED,
        7 => EMFILE,
        8 => EAFNOSUPPORT,
        9 => EINVAL,
        11 => EAGAIN,
        12 => ENOTCONN,
        _ => EIO,
    }
}

pub fn nskt_call(op: u16, payload: &[u8], resp: &mut [u8]) -> Result<(u16, usize)> {
    let mut req = Vec::with_capacity(HDR_LEN + payload.len());
    req.extend_from_slice(&NSKT_MAGIC.to_le_bytes());
    req.extend_from_slice(&1u16.to_le_bytes());
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&[0u8; 4]);
    req.extend_from_slice(&1u32.to_le_bytes());
    req.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    req.extend_from_slice(payload);
    let ret = unsafe {
        syscall6(MK_IPC_CALL, NSKT_ENDPOINT, req.as_ptr() as u64, req.len() as u64,
                 resp.as_mut_ptr() as u64, resp.len() as u64, 0)
    };
    if ret < HDR_LEN as i64 { return Err(Errno(EIO)); }
    let errno = u16::from_le_bytes([resp[8], resp[9]]);
    let len = u32::from_le_bytes([resp[16], resp[17], resp[18], resp[19]]) as usize;
    Ok((errno, len.min(ret as usize - HDR_LEN)))
}
