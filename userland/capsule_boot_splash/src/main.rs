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
    heap_init, mk_attest_status, mk_exit, mk_ipc_recv_from, mk_surface_release, mk_uptime_ms,
    mk_yield, AttestStatus, INPUT_KIND_KEY_DOWN,
};

// Linger on the splash until the desktop shell registers (the desktop is
// coming up), then a short settle so it paints behind this overlay before
// we hand off. MAX_DWELL is a hard cap so the splash never hangs if the
// shell fails to appear.
const SETTLE_MS: i64 = 1000;
const MAX_DWELL_MS: i64 = 30_000;
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
    grabbed_interact(comp, base, w, h, stride, &att, badge);
    let _ = scene::remove(comp, 5);
    let _ = mk_surface_release(handle);
    0
}

fn grabbed_interact(comp: u32, base: u64, w: u32, h: u32, stride: u32, att: &AttestStatus, badge: Option<bool>) {
    let router = match proto::lookup(b"input_router") {
        Some(rp) => rp,
        None => return interact(comp, base, w, h, stride, att, badge),
    };
    let _ = input::subscribe(router, 10);
    let _ = input::grab(router, 11);
    interact(comp, base, w, h, stride, att, badge);
    let _ = input::release(router, 12);
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
    // Elapsed time for the dwell and settle gates comes from the monotonic
    // uptime, not the wall clock. mk_time_millis returns -61 until the wall
    // clock has an epoch anchor, and a boot that has not synced one yet would
    // pin now at a negative value: the settle and MAX_DWELL gates could then
    // never fire and the splash would sit on "initializing kernel gui" until
    // MAX_ITERS, which is effectively never. Uptime is always valid from the
    // first read, so the handoff happens whether or not the epoch is known.
    let mut start = mk_uptime_ms();
    let mut desktop_up_at: i64 = -1;
    loop {
        let now = mk_uptime_ms();
        if start < 0 {
            start = now;
        }
        let el = if start >= 0 && now >= start { now - start } else { 0 };
        if desktop_up_at < 0 && proto::lookup(b"desktop_shell").is_some() {
            desktop_up_at = now;
        }
        let handed_off =
            desktop_up_at >= 0 && now >= desktop_up_at && now - desktop_up_at >= SETTLE_MS;
        if (!show_detail && (handed_off || el >= MAX_DWELL_MS)) || iters >= MAX_ITERS {
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

