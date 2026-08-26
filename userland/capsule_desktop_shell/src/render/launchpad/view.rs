// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The one ordered list of Launchpad entries that is on screen, so the
//! painter and the hit-test can never disagree about what a cell holds.

use super::grid::per_page;
use super::hit::{target, Target};
use crate::state::{Context, LAUNCHER_APPS, TOOL_APPS};

fn matches(label: &[u8], query: &str) -> bool {
    if query.is_empty() {
        return true;
    }
    let q = query.as_bytes();
    if q.len() > label.len() {
        return false;
    }
    let fold = |b: u8| b.to_ascii_lowercase();
    label
        .windows(q.len())
        .any(|w| w.iter().map(|&b| fold(b)).eq(q.iter().map(|&b| fold(b))))
}

pub(crate) fn label_of(ctx: &Context, t: Target) -> &[u8] {
    match t {
        Target::App(a) => LAUNCHER_APPS[a].label,
        Target::Tool(t) => TOOL_APPS[t].label,
        Target::Installed(i) => ctx
            .installed_apps
            .get(i)
            .map(|n| n.as_slice())
            .unwrap_or(b""),
        Target::Package(i) => ctx
            .pkg_files
            .get(i)
            .map(|n| n.as_bytes())
            .unwrap_or(b""),
    }
}

pub(super) fn rebuild(ctx: &mut Context) {
    let installed = ctx.installed_apps.len();
    let packages = ctx.pkg_files.len();
    let total = LAUNCHER_APPS.len() + TOOL_APPS.len() + installed + packages;
    let mut next = alloc::vec::Vec::with_capacity(total);
    for i in 0..total {
        let t = target(i, installed, packages);
        if matches(label_of(ctx, t), &ctx.launchpad_query) {
            next.push(t);
        }
    }
    ctx.launchpad_view = next;
    let last = pages(ctx).saturating_sub(1);
    if ctx.launchpad_page > last {
        ctx.launchpad_page = last;
    }
}

pub(super) fn pages(ctx: &Context) -> usize {
    let cap = per_page(ctx.width, ctx.height).max(1);
    ctx.launchpad_view.len().div_ceil(cap).max(1)
}

pub(super) fn page_slice(ctx: &Context) -> &[Target] {
    let cap = per_page(ctx.width, ctx.height).max(1);
    let start = ctx.launchpad_page * cap;
    let end = (start + cap).min(ctx.launchpad_view.len());
    if start >= ctx.launchpad_view.len() {
        return &[];
    }
    &ctx.launchpad_view[start..end]
}
