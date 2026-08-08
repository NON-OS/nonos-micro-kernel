// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! While the install prompt is up it swallows every click: Approve commits the
//! package against the digest the query returned, anything else dismisses it.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::{approve_rect, geometry::hit};
use crate::render::sync_toast_layer;
use crate::server::repaint::repaint;
use crate::state::{Context, NotifyLevel, PkgInstallPrompt};

pub(crate) fn click(ctx: &mut Context, px: u32, py: u32) -> bool {
    if ctx.pending_pkg_install.is_none() {
        return false;
    }
    if hit(approve_rect(ctx.width, ctx.height), px, py) {
        if let Some(prompt) = ctx.pending_pkg_install.take() {
            commit(ctx, prompt);
        }
    } else {
        ctx.pending_pkg_install = None;
    }
    repaint(ctx);
    true
}

/// The commit re-verifies against the digest the prompt was built from, so a
/// file swapped between the query and the click fails rather than installing
/// something the user never saw.
fn commit(ctx: &mut Context, prompt: PkgInstallPrompt) {
    let mut text = Vec::with_capacity(32);
    let level = match crate::installer_client::pkg_commit(&prompt.path, &prompt.summary.digest) {
        Ok(()) => {
            text.extend_from_slice(b"installed ");
            text.extend_from_slice(&prompt.summary.slug);
            NotifyLevel::Info
        }
        Err(code) => {
            text.extend_from_slice(b"install failed: ");
            crate::server::handlers::pkg_install::push_i32(&mut text, code);
            NotifyLevel::Error
        }
    };
    ctx.toasts.push(&text, level, mk_time_millis());
    sync_toast_layer(ctx);
}
