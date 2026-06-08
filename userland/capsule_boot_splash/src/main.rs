#![no_std]
#![no_main]

extern crate alloc;

mod chrome;
mod detail;
mod display;
mod input;
mod paint;
mod proto;
mod scene;
mod surface;
mod vignette;

use nonos_libc::{
    heap_init, mk_attest_status, mk_exit, mk_ipc_recv_from, mk_surface_release, mk_time_millis,
    mk_yield, AttestStatus, INPUT_KIND_KEY_DOWN,
};

const DWELL_MS: i64 = 2500;
const MAX_ITERS: u32 = 8_000_000;
const READY_ATTEMPTS: u32 = 256;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    mk_exit(run())
}

fn run() -> i32 {
    let comp = match wait_compositor() {
        Some(p) => p,
        None => return 2,
    };
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

fn wait_compositor() -> Option<u32> {
    for _ in 0..READY_ATTEMPTS {
        if let Some(p) = proto::lookup(b"compositor") {
            if proto::healthcheck(p, 1).is_ok() {
                return Some(p);
            }
        }
        mk_yield();
    }
    None
}

fn interact(comp: u32, base: u64, w: u32, h: u32, stride: u32, att: &AttestStatus, badge: Option<bool>) {
    let spx = stride as usize / 4;
    let mut rx = [0u8; 64];
    let mut sender = 0u32;
    let mut show_detail = false;
    let mut iters: u32 = 0;
    let mut frame: u32 = u32::MAX;
    let mut start = mk_time_millis();
    loop {
        let now = mk_time_millis();
        if start < 0 {
            start = now;
        }
        let el = if start >= 0 && now >= start { now - start } else { 0 };
        if (!show_detail && el >= DWELL_MS) || iters >= MAX_ITERS {
            break;
        }
        iters = iters.saturating_add(1);
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 50, &mut sender);
        if n > 0 {
            if let Some((kind, code)) = input::parse_key(&rx[..n as usize]) {
                if kind == INPUT_KIND_KEY_DOWN {
                    show_detail = (code == b'D' as u32 || code == b'd' as u32) && !show_detail;
                    if show_detail {
                        detail::detail(base, w, h, stride, att);
                    } else {
                        paint::splash(base, w, h, stride, badge);
                    }
                    let _ = scene::damage(comp, 20, w, h);
                }
            }
            continue;
        }
        if !show_detail {
            let f = (el / 150) as u32;
            if f != frame {
                frame = f;
                let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
                paint::status(buf, spx, w, h, frame);
                let _ = scene::damage(comp, 21, w, h);
            }
        }
        mk_yield();
    }
}

