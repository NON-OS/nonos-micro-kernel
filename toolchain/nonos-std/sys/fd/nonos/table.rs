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

// The descriptor table: a fixed array of slots mapping each live `RawFd` to
// the service handle that backs it. Fixed so a runaway descriptor leak fails
// closed instead of growing without bound, and sized generously above what a
// CLI process opens.

use crate::io::{self, Error, ErrorKind};
use crate::sync::{Mutex, MutexGuard};

use super::ipc;

// Descriptors 0..3 are the stdio streams and never occupy a table slot.
const FIRST_FD: usize = 3;
const MAX_FDS: usize = 1024;

// What a descriptor stands for. `Socket` is a net.sockets stream or datagram
// handle; `File` is a vfs handle together with the service port and pid its
// close call must carry. Copy so a lookup never holds the table lock while
// the caller uses the result; Eq so close can tell whether another descriptor
// still shares the same backing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Backing {
    Socket { handle: u32 },
    File { port: u32, pid: u32, handle: u32 },
}

static TABLE: Mutex<[Option<Backing>; MAX_FDS]> = Mutex::new([None; MAX_FDS]);

// The table is process-local bookkeeping; a panic while a slot is half
// updated cannot poison anything an unwinding thread could observe torn, so
// recover the guard rather than propagate the poison.
fn slots() -> MutexGuard<'static, [Option<Backing>; MAX_FDS]> {
    match TABLE.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

// Register `backing` and return the descriptor assigned to it. The lowest
// free slot at or above `FIRST_FD` is chosen, matching the "lowest available
// fd" contract fd-based code expects.
pub fn install(backing: Backing) -> io::Result<i32> {
    let mut table = slots();
    for fd in FIRST_FD..MAX_FDS {
        if table[fd].is_none() {
            table[fd] = Some(backing);
            return Ok(fd as i32);
        }
    }
    Err(Error::new(ErrorKind::Other, "descriptor table full"))
}

// The backing for `fd`, if it names a live descriptor. Stdio and out-of-range
// or free descriptors return None; callers treat that as "not a socket/file".
pub fn get(fd: i32) -> Option<Backing> {
    let idx = usize::try_from(fd).ok()?;
    if !(FIRST_FD..MAX_FDS).contains(&idx) {
        return None;
    }
    slots()[idx]
}

// The net.sockets handle behind `fd`, if it names a live socket descriptor.
// This is the one crossing an fd-based reactor (a vendored mio backend) needs:
// it registers a `RawFd` for readiness polling and must resolve it to the
// service handle OP_POLL takes. Returns None for stdio, files, or free slots.
pub fn socket_handle(fd: i32) -> Option<u32> {
    match get(fd) {
        Some(Backing::Socket { handle }) => Some(handle),
        _ => None,
    }
}

// Install an already-open net.sockets `handle` and return the `RawFd` assigned
// to it. The other half of the fd-based-reactor seam: a vendored mio backend
// creates sockets by talking to net.sockets directly (socket/accept), then
// registers the resulting handle here so std's `TcpStream`/`UdpSocket` read and
// write flow through the table exactly as a PAL-created socket does.
pub fn register_socket(handle: u32) -> io::Result<i32> {
    install(Backing::Socket { handle })
}

// Point a new descriptor at the same backing as `fd` (the dup contract). The
// duplicates share one service handle, which is released when the last of
// them closes.
pub fn dup_fd(fd: i32) -> io::Result<i32> {
    match get(fd) {
        Some(backing) => install(backing),
        None => Err(Error::new(ErrorKind::InvalidInput, "unknown descriptor")),
    }
}

// Release `fd`: free its slot, and issue the backing's close IPC only when no
// other descriptor still shares the same backing, so duplicated descriptors
// behave like dup'd fds over one open resource. Closing stdio or an unknown
// descriptor is a no-op, which is what drop-time close paths want.
pub fn close_fd(fd: i32) {
    let idx = match usize::try_from(fd) {
        Ok(i) if (FIRST_FD..MAX_FDS).contains(&i) => i,
        _ => return,
    };
    let last = {
        let mut table = slots();
        match table[idx].take() {
            Some(backing) if !table.iter().any(|s| *s == Some(backing)) => Some(backing),
            _ => None,
        }
    };
    if let Some(backing) = last {
        ipc::close_backing(backing);
    }
}
