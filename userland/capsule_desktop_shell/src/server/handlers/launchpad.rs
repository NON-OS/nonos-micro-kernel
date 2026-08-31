// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Opening the Launchpad from the dock, and acting on a click while it is open:
//! a tile launches its app, tool or capsule-store app, a click on the search
//! pill or the page dots belongs to that chrome, and a click on empty space
//! dismisses the overlay. A store app is absent from the kernel's compile-time
//! spawn table, so it goes to the installer rather than the dock's spawn path.

use crate::render::launchpad::{dots_hit, hit_target, page_slice, rebuild, search_hit, Target};
use crate::server::handlers::launcher_request;
use crate::server::repaint::repaint;
use crate::state::{Context, LAUNCHER_APPS};

/// KEY_DOWN kind bit, grabbed for the overlay's lifetime so keys typed into the
/// search field reach the shell rather than whatever window sits behind it.
const KEY_DOWN_BIT: u32 = 1;

pub fn open(ctx: &mut Context) {
    ctx.launchpad = true;
    ctx.launchpad_query.clear();
    ctx.launchpad_page = 0;
    rebuild(ctx);
    let rid = ctx.issue_request_id();
    let _ = crate::input_router_client::grab(ctx.input_router_port, rid, KEY_DOWN_BIT);
    repaint(ctx);
}

pub fn close(ctx: &mut Context) {
    ctx.launchpad = false;
    let rid = ctx.issue_request_id();
    let _ = crate::input_router_client::release_grab(ctx.input_router_port, rid);
    ctx.launchpad_query.clear();
    ctx.launchpad_page = 0;
    repaint(ctx);
}

pub fn click(ctx: &mut Context, px: u32, py: u32) {
    if let Some(page) = dots_hit(ctx, px, py) {
        ctx.launchpad_page = page;
        repaint(ctx);
        return;
    }
    if search_hit(ctx, px, py) {
        return;
    }
    match hit_target(ctx, px, py) {
        Some(t) => launch(ctx, t),
        None => close(ctx),
    }
}

/// Enter runs the first entry the current filter left standing, so a search can
/// be completed without reaching for the pointer.
pub fn launch_first(ctx: &mut Context) {
    let first = page_slice(ctx).first().copied();
    if let Some(t) = first {
        launch(ctx, t);
    }
}

fn launch(ctx: &mut Context, t: Target) {
    match t {
        Target::App(a) => {
            let _ = launcher_request::request(&LAUNCHER_APPS[a]);
        }
        Target::Tool(_) => {
            // A tool is a command-line program: it runs in the terminal,
            // where the kernel spawns it parented to the shell so its output
            // streams into the scrollback. Opening the terminal is the
            // launch; the user runs the tool there by name.
            let _ = launcher_request::request_service(b"app.terminal");
        }
        Target::Installed(i) => {
            if let Some(name) = ctx.installed_apps.get(i).cloned() {
                ctx.pending_consent = Some(name);
            }
        }
        Target::Package(i) => super::pkg_install::begin(ctx, i),
    }
    close(ctx);
}
