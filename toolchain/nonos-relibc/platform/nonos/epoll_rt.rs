use core::cell::SyncUnsafeCell;

use super::super::types::c_int;

const EPOLL_FD_BASE: c_int = 512;
const INSTANCES: usize = 16;
const MAX_REGS: usize = 32;

#[derive(Clone, Copy)]
struct Reg { fd: c_int, events: u32, data: u64, used: bool }

#[derive(Clone, Copy)]
struct Instance { used: bool, regs: [Reg; MAX_REGS] }

const EMPTY_REG: Reg = Reg { fd: 0, events: 0, data: 0, used: false };
const EMPTY_INST: Instance = Instance { used: false, regs: [EMPTY_REG; MAX_REGS] };

static EPOLL: SyncUnsafeCell<[Instance; INSTANCES]> = SyncUnsafeCell::new([EMPTY_INST; INSTANCES]);

fn table() -> &'static mut [Instance; INSTANCES] { unsafe { &mut *EPOLL.get() } }

pub fn is_epoll_fd(fd: c_int) -> bool {
    fd >= EPOLL_FD_BASE && (fd - EPOLL_FD_BASE) < INSTANCES as c_int
}
fn idx(fd: c_int) -> Option<usize> {
    if is_epoll_fd(fd) { Some((fd - EPOLL_FD_BASE) as usize) } else { None }
}
fn live(fd: c_int) -> Option<usize> {
    let i = idx(fd)?;
    if table()[i].used { Some(i) } else { None }
}

pub fn create() -> Option<c_int> {
    for (i, inst) in table().iter_mut().enumerate() {
        if !inst.used { *inst = Instance { used: true, regs: [EMPTY_REG; MAX_REGS] }; return Some(EPOLL_FD_BASE + i as c_int); }
    }
    None
}
pub fn destroy(fd: c_int) {
    if let Some(i) = idx(fd) { table()[i].used = false; }
}
pub fn set(epfd: c_int, fd: c_int, events: u32, data: u64) -> bool {
    let Some(i) = live(epfd) else { return false; };
    for r in table()[i].regs.iter_mut() {
        if r.used && r.fd == fd { r.events = events; r.data = data; return true; }
    }
    for r in table()[i].regs.iter_mut() {
        if !r.used { *r = Reg { fd, events, data, used: true }; return true; }
    }
    false
}
pub fn remove(epfd: c_int, fd: c_int) -> bool {
    let Some(i) = live(epfd) else { return false; };
    for r in table()[i].regs.iter_mut() {
        if r.used && r.fd == fd { r.used = false; return true; }
    }
    false
}
pub fn for_each<F: FnMut(c_int, u32, u64)>(epfd: c_int, mut f: F) -> bool {
    let Some(i) = live(epfd) else { return false; };
    for r in table()[i].regs { if r.used { f(r.fd, r.events, r.data); } }
    true
}
