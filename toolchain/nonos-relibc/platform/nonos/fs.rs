use alloc::vec::Vec;
use core::cell::SyncUnsafeCell;
use crate::{error::{Errno, Result}, header::errno::EIO};
use super::{super::types::c_int, lowlevel::{syscall6, MK_IPC_CALL}};

const VFS_ENDPOINT: u64 = 4104;
const VFS_MAGIC: u32 = 0x4E4F_5646;
const VFS_VERSION: u16 = 1;
const HDR_LEN: usize = 20;
pub const OP_OPEN: u16 = 1;
pub const OP_CLOSE: u16 = 2;
pub const OP_READ: u16 = 3;
pub const OP_WRITE: u16 = 4;
pub const OP_STAT: u16 = 5;
pub const OP_MKDIR: u16 = 8;
pub const OP_UNLINK: u16 = 9;
pub const OP_RENAME: u16 = 10;
pub const OP_SEEK: u16 = 11;
pub const OP_PREAD: u16 = 12;
pub const OP_PWRITE: u16 = 13;
pub const VFS_O_CREATE: u32 = 1 << 0;
pub const VFS_O_TRUNC: u32 = 1 << 1;
pub const VFS_O_APPEND: u32 = 1 << 2;

const FD_SLOTS: usize = 64;
static FD_TABLE: SyncUnsafeCell<[i32; FD_SLOTS]> = SyncUnsafeCell::new([-1; FD_SLOTS]);
fn table() -> &'static mut [i32; FD_SLOTS] { unsafe { &mut *FD_TABLE.get() } }

pub fn fd_alloc(vfs_fd: u32) -> Option<c_int> {
    for (i, s) in table().iter_mut().enumerate() {
        if *s < 0 { *s = vfs_fd as i32; return Some((i + 3) as c_int); }
    }
    None
}
pub fn fd_vfs(fd: c_int) -> Option<u32> {
    let i = fd.checked_sub(3)? as usize;
    if i >= FD_SLOTS { return None; }
    let v = table()[i];
    if v < 0 { None } else { Some(v as u32) }
}
pub fn fd_free(fd: c_int) {
    if let Some(i) = fd.checked_sub(3).map(|v| v as usize) {
        if i < FD_SLOTS { table()[i] = -1; }
    }
}
pub fn fd_dup(fd: c_int) -> Option<c_int> { fd_alloc(fd_vfs(fd)?) }
pub fn fd_set(fd: c_int, vfs_fd: u32) -> bool {
    match fd.checked_sub(3).map(|v| v as usize) {
        Some(i) if i < FD_SLOTS => { table()[i] = vfs_fd as i32; true }
        _ => false,
    }
}
pub fn vfs_call(op: u16, payload: &[u8], resp: &mut [u8]) -> Result<(i32, usize)> {
    let mut req = Vec::with_capacity(HDR_LEN + payload.len());
    req.extend_from_slice(&VFS_MAGIC.to_le_bytes());
    req.extend_from_slice(&VFS_VERSION.to_le_bytes());
    req.extend_from_slice(&op.to_le_bytes());
    req.extend_from_slice(&[0u8; 8]);
    req.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    req.extend_from_slice(payload);
    let ret = unsafe {
        syscall6(MK_IPC_CALL, VFS_ENDPOINT,
                 req.as_ptr() as u64, req.len() as u64,
                 resp.as_mut_ptr() as u64, resp.len() as u64, 0)
    };
    if ret < 24 { return Err(Errno(EIO)); }
    let s = i32::from_le_bytes([resp[20], resp[21], resp[22], resp[23]]);
    Ok((s, (ret as usize) - 24))
}
