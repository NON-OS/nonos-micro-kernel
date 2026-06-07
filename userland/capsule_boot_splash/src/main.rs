#![no_std]
#![no_main]

extern crate alloc;

mod detail;
mod display;
mod input;
mod paint;
mod proto;
mod scene;
mod surface;

use nonos_libc::{
    heap_init, mk_attest_status, mk_exit, mk_ipc_recv_from, mk_surface_release, AttestStatus,
    INPUT_KIND_KEY_DOWN,
};

const RECV_TIMEOUT_MS: u64 = 100;
const IDLE_TICKS: u32 = 30;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run())
}

fn run() -> i32 {
    let comp = match proto::lookup(b"compositor") {
        Some(p) => p,
        None => return 2,
    };
    if proto::healthcheck(comp, 1).is_err() {
        return 3;
    }
    let (w, h, stride) = match display::query(comp, 2) {
        Ok(d) => d,
        Err(_) => return 4,
    };
    let (base, handle) = match surface::setup(w, h, stride) {
        Some(s) => s,
        None => return 5,
    };
    let mut att = AttestStatus::default();
    let badge = if mk_attest_status(&mut att) == 0 { Some(att.zk_verified == 1) } else { None };
    paint::splash(base, w, h, stride, badge);
    if scene::submit(comp, 3, handle, w, h).is_err() {
        let _ = mk_surface_release(handle);
        return 6;
    }
    let _ = scene::damage(comp, 4, w, h);
    let router = proto::lookup(b"input_router");
    if let Some(rp) = router {
        let _ = input::subscribe(rp, 10);
        let _ = input::grab(rp, 11);
    }
    interact(comp, base, w, h, stride, &att, badge);
    if let Some(rp) = router {
        let _ = input::release(rp, 12);
    }
    let _ = scene::remove(comp, 5);
    let _ = mk_surface_release(handle);
    0
}

fn interact(comp: u32, base: u64, w: u32, h: u32, stride: u32, att: &AttestStatus, badge: Option<bool>) {
    let mut rx = [0u8; 64];
    let mut sender = 0u32;
    let mut show_detail = false;
    let mut idle = 0u32;
    while idle < IDLE_TICKS {
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), RECV_TIMEOUT_MS, &mut sender);
        if n <= 0 {
            idle += 1;
            continue;
        }
        idle = 0;
        let Some((kind, code)) = input::parse_key(&rx[..n as usize]) else {
            continue;
        };
        if kind != INPUT_KIND_KEY_DOWN {
            continue;
        }
        show_detail = (code == b'D' as u32 || code == b'd' as u32) && !show_detail;
        if show_detail {
            detail::detail(base, w, h, stride, att);
        } else {
            paint::splash(base, w, h, stride, badge);
        }
        let _ = scene::damage(comp, 20, w, h);
    }
}
