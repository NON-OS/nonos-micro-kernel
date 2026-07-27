// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The list of embedded tool capsules. The block between the generated markers
//! is written by `tools/nonos-app` from `userland/apps.list`, the single
//! source of truth. Adding a tool is `nonos-app add <crate>`, not a hand edit.

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use super::spec::ToolCapsule;
use crate::sys::boot_log;

/// Every embedded tool capsule, generated from `userland/apps.list`.
fn embedded_tools() -> Vec<ToolCapsule> {
    vec![
        // nonos-app:begin (generated; do not edit by hand)
        tool_capsule!("tool.grex", 4900, "endpoint.tool.grex.reply", 4901,
            "../../../target/upstream-grex/bin/grex", "grex"),
        tool_capsule!("tool.dotenv-linter", 4902, "endpoint.tool.dotenv-linter.reply", 4903,
            "../../../target/upstream-dotenv-linter/bin/dotenv-linter", "dotenv-linter"),
        tool_capsule!("tool.pastel", 4904, "endpoint.tool.pastel.reply", 4905,
            "../../../target/upstream-pastel/bin/pastel", "pastel"),
        tool_capsule!("tool.jsonxf", 4906, "endpoint.tool.jsonxf.reply", 4907,
            "../../../target/upstream-jsonxf/bin/jsonxf", "jsonxf"),
        tool_capsule!("tool.choose", 4908, "endpoint.tool.choose.reply", 4909,
            "../../../target/upstream-choose/bin/choose", "choose"),
        tool_capsule!("tool.tokei", 4910, "endpoint.tool.tokei.reply", 4911,
            "../../../target/upstream-tokei/bin/tokei", "tokei"),
        tool_capsule!("tool.huniq", 4912, "endpoint.tool.huniq.reply", 4913,
            "../../../target/upstream-huniq/bin/huniq", "huniq"),
        tool_capsule!("tool.csview", 4914, "endpoint.tool.csview.reply", 4915,
            "../../../target/upstream-csview/bin/csview", "csview"),
        // nonos-app:end
    ]
}

/// Spawn every embedded tool capsule, verified under the baked trust anchor. One
/// tool failing to spawn is logged and skipped so it never blocks the others.
pub fn spawn_all() {
    for tool in embedded_tools() {
        match tool.spawn() {
            Ok(()) => boot_log::ok("TOOL", tool.name),
            Err(_) => boot_log::error("tool capsule spawn failed"),
        }
    }
}
