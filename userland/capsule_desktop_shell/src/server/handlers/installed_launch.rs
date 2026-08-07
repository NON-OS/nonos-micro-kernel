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

//! Launch a capsule-store app from the Launchpad. The kernel's spawn-instance
//! path resolves a service name against a compile-time table of embedded
//! capsules, so it can never reach an app installed at runtime; the installer
//! owns that load instead. The pid it returns is remembered, so a later click
//! raises the running window rather than loading a second copy.

use alloc::vec::Vec;

use nonos_libc::{mk_ipc_send_to_pid, mk_service_lookup, mk_time_millis, mk_yield};

use super::launcher_request::focus_frame;
use crate::installer_client::load_by_name;
use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel};

pub fn launch(ctx: &mut Context, name: &[u8]) {
    if let Some(pid) = ctx.installed_pids.get(name).copied() {
        if focus(pid) {
            return;
        }
        ctx.installed_pids.remove(name);
    }
    if let Some(pid) = already_running(name) {
        ctx.installed_pids.insert(name.to_vec(), pid);
        if focus(pid) {
            return;
        }
        ctx.installed_pids.remove(name);
    }
    match load_by_name(name) {
        Ok(pid) => {
            ctx.installed_pids.insert(name.to_vec(), pid);
            boot(pid);
        }
        Err(ERR_EXIST) => focus_running(ctx, name),
        Err(status) => report_failure(ctx, status),
    }
}

const ERR_REJECTED: i32 = -13;
/// The kernel refuses a second spawn on a service endpoint a live instance
/// still holds, and the installer forwards that as this errno. It means the
/// app is already running, not that the launch failed, so it must raise the
/// running window instead of the failure toast.
const ERR_EXIST: i32 = -17;

/// A collision names an instance the top-of-launch registry probe missed --
/// another client loaded it in the interim. Re-probe and focus it; if the
/// window is already gone, stay silent rather than cry a false failure.
fn focus_running(ctx: &mut Context, name: &[u8]) {
    if let Some(pid) = already_running(name) {
        ctx.installed_pids.insert(name.to_vec(), pid);
        if focus(pid) {
            return;
        }
        ctx.installed_pids.remove(name);
    }
}

/// A refused load used to look exactly like nothing happening; say why on
/// screen, and call out a verification rejection distinctly since it means
/// the store artifacts themselves failed attestation.
fn report_failure(ctx: &mut Context, status: i32) {
    let text: &[u8] = if status == ERR_REJECTED {
        b"app rejected: failed verification"
    } else {
        b"app failed to launch"
    };
    ctx.toasts.push(text, NotifyLevel::Error, mk_time_millis());
    sync_toast_layer(ctx);
}

// The kernel registers the app's `proc.<pid>` inbox inside the load syscall
// itself, before the pid travels back here, so the first send normally lands.
// The extra attempts only cover a momentarily full queue, and cap this click's
// stall at two yields, because the shell services its frame loop on this thread.
const FOCUS_ATTEMPTS: u32 = 3;

/// Deliver the focus frame a freshly loaded app blocks on before it builds its
/// window. Without it the app attests, runs, and then waits forever, so nothing
/// appears until a second click takes the already-running path above.
fn boot(pid: u32) {
    if focus(pid) {
        return;
    }
    for _ in 1..FOCUS_ATTEMPTS {
        mk_yield();
        if focus(pid) {
            return;
        }
    }
}

/// Another client -- the terminal's `nox install`, say -- may already have
/// loaded the app, and this shell would know nothing about it. Spawning a
/// second copy then dies in the kernel on the service endpoint the live
/// instance still holds, with no marker and no reply, so ask the registry
/// before loading.
fn already_running(name: &[u8]) -> Option<u32> {
    let mut service = Vec::with_capacity(b"app.".len() + name.len());
    service.extend_from_slice(b"app.");
    service.extend_from_slice(name);
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

/// A send failure means the pid is gone or no longer takes control frames, so
/// the caller falls back to loading the app afresh.
fn focus(pid: u32) -> bool {
    let frame = focus_frame();
    mk_ipc_send_to_pid(pid, frame.as_ptr(), frame.len()) >= 0
}
