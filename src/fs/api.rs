// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;

use super::fd;
use alloc::collections::BTreeSet;
use spin::Mutex;

static PIPE_FDS: Mutex<BTreeSet<i32>> = Mutex::new(BTreeSet::new());

// `crate::network::unix::UnixSocket` lives in the legacy network tree.
// The microkernel exposes IPC over `nonos_channel`/`nonos_inbox`; AF_UNIX
// is not part of the trusted-path syscall surface.

pub struct FdInfo {
    pub path: alloc::string::String,
    pub flags: u32,
    pub mode: u32,
    pub position: u64,
    pub mount_id: u32,
    pub inode: u64,
}

pub fn allocate_fd() -> Result<i32, i32> {
    fd::fd_open_raw(core::ptr::null(), 0).map_err(|_| -24)
}

pub fn pread(fd: i32, buf: &mut [u8], offset: u64) -> Result<usize, i32> {
    let orig_pos = fd::fd_get_offset(fd).map_err(|_| -9i32)? as i64;
    fd::fd_lseek(fd, offset as i64, 0).map_err(|_| -9i32)?;
    let result = fd::fd_read(fd, buf.as_mut_ptr(), buf.len());
    let _ = fd::fd_lseek(fd, orig_pos, 0);
    result.map_err(|_| -5)
}

pub fn pwrite(fd: i32, buf: &[u8], offset: u64) -> Result<usize, i32> {
    let orig_pos = fd::fd_get_offset(fd).map_err(|_| -9i32)? as i64;
    fd::fd_lseek(fd, offset as i64, 0).map_err(|_| -9i32)?;
    let result = fd::fd_write(fd, buf.as_ptr(), buf.len());
    let _ = fd::fd_lseek(fd, orig_pos, 0);
    result.map_err(|_| -5)
}

pub fn get_file_size(fd: i32) -> Result<u64, i32> {
    let mut stat_buf = [0u8; 48];
    fd::fd_fstat(fd, stat_buf.as_mut_ptr()).map_err(|_| -9)?;
    let size = u64::from_le_bytes(stat_buf[16..24].try_into().unwrap_or([0; 8]));
    Ok(size)
}

pub fn register_pipe_reader<T>(_fd: i32, _reader: T) {
    PIPE_FDS.lock().insert(_fd);
}
pub fn register_pipe_writer<T>(_fd: i32, _writer: T) {
    PIPE_FDS.lock().insert(_fd);
}
pub fn set_cloexec(fd: i32, cloexec: bool) {
    let _ = fd::fd_set_cloexec(fd, cloexec);
}
pub fn is_pipe_fd(fd: i32) -> bool {
    PIPE_FDS.lock().contains(&fd)
}
pub fn unregister_pipe_fd(fd: i32) {
    PIPE_FDS.lock().remove(&fd);
}

pub fn get_pipe_buffer_size(_fd: i32) -> Result<usize, i32> {
    Ok(crate::fs::pipe::PIPE_BUF_SIZE)
}

pub fn set_pipe_buffer_size(_fd: i32, size: usize) -> Result<usize, i32> {
    Ok(size.min(crate::fs::pipe::PIPE_BUF_SIZE * 16))
}

pub fn get_process_fds(pid: i32) -> Result<alloc::vec::Vec<i32>, i32> {
    let proc = crate::process::get_process(pid as u32).ok_or(-3)?;
    Ok(proc.fd_table.all_fds())
}

pub fn get_process_fd(pid: i32, fd: i32) -> Option<FdInfo> {
    let proc = crate::process::get_process(pid as u32)?;
    let entry = proc.fd_table.get(fd)?;
    Some(FdInfo {
        path: alloc::format!("fd:{}", fd),
        flags: entry.flags,
        mode: 0,
        position: 0,
        mount_id: 0,
        inode: entry.internal_id as u64,
    })
}
