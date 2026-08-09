// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Clicking a Launchpad package tile asks the installer to verify the file
//! and, on success, raises the install-consent modal with the attested
//! summary; a rejection is said out loud instead of doing nothing.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use crate::render::sync_toast_layer;
use crate::state::{Context, NotifyLevel, PkgInstallPrompt};

pub fn begin(ctx: &mut Context, index: usize) {
    let Some(name) = ctx.pkg_files.get(index) else { return };
    let mut path = Vec::with_capacity(b"/pkgs/".len() + name.len());
    path.extend_from_slice(b"/pkgs/");
    path.extend_from_slice(name.as_bytes());
    match crate::installer_client::pkg_query(&path) {
        Ok(summary) => ctx.pending_pkg_install = Some(PkgInstallPrompt { path, summary }),
        Err(code) => report_rejected(ctx, code),
    }
}

/// A refused package must never look like a dead click: the installer's errno
/// is the only thing distinguishing a bad signature from a missing file.
fn report_rejected(ctx: &mut Context, code: i32) {
    let mut text = Vec::with_capacity(32);
    text.extend_from_slice(b"package rejected: ");
    push_i32(&mut text, code);
    ctx.toasts.push(&text, NotifyLevel::Error, mk_time_millis());
    sync_toast_layer(ctx);
}

pub(crate) fn push_i32(out: &mut Vec<u8>, v: i32) {
    let mut n = v as i64;
    if n < 0 {
        out.push(b'-');
        n = -n;
    }
    let mut digits = [0u8; 10];
    let mut i = digits.len();
    loop {
        i -= 1;
        digits[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    out.extend_from_slice(&digits[i..]);
}
