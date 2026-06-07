#![no_std]
#![no_main]

extern crate alloc;

mod display;
mod paint;
mod proto;
mod scene;
mod surface;

use nonos_libc::{heap_init, mk_attest_status, mk_exit, mk_surface_release, mk_yield, AttestStatus};

const DWELL_YIELDS: u32 = 60_000;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run())
}

fn run() -> i32 {
    let port = match proto::lookup_compositor() {
        Some(p) => p,
        None => return 2,
    };
    if proto::healthcheck(port, 1).is_err() {
        return 3;
    }
    let (w, h, stride) = match display::query(port, 2) {
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
    if scene::submit(port, 3, handle, w, h).is_err() {
        let _ = mk_surface_release(handle);
        return 6;
    }
    let _ = scene::damage(port, 4, w, h);
    dwell();
    let _ = scene::remove(port, 5);
    let _ = mk_surface_release(handle);
    0
}

fn dwell() {
    let mut i = 0u32;
    while i < DWELL_YIELDS {
        mk_yield();
        i += 1;
    }
}
