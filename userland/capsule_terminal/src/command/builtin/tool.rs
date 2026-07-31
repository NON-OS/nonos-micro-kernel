// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Run one of the baked, attested command-line tools (grex, tokei, csview, ...)
//! from the shell. The kernel spawns the tool parented to this terminal, so the
//! same async drain job that streams a store install's output streams the
//! tool's stdout here. Adding a tool is one line in `TOOLS`.

use alloc::vec::Vec;
use nonos_libc::mk_tool_run;

use crate::command::builtin::nox::install::InstallJob;
use crate::term::state::State;

/// The installed command-line tools, by the bare name typed at the prompt. Each
/// maps to its baked `tool.<name>` service. Kept in step with userland/apps.list.
pub const TOOLS: &[&[u8]] =
    &[b"grex", b"dotenv-linter", b"pastel", b"jsonxf", b"tokei", b"huniq", b"csview"];

pub fn is_tool(name: &[u8]) -> bool {
    TOOLS.contains(&name)
}

/// Spawn the baked tool named by `args[0]` with the rest as its argv, and return
/// a drain job that streams its stdout to the terminal. `None` on a spawn error,
/// with the reason already pushed to the scrollback.
pub fn prepare(state: &mut State, args: &[&[u8]]) -> Option<InstallJob> {
    let name = args[0];
    let mut service = Vec::with_capacity(5 + name.len());
    service.extend_from_slice(b"tool.");
    service.extend_from_slice(name);

    let argv = argv_blob(args);
    let rc = mk_tool_run(&service, &argv);
    if rc < 0 {
        state.scrollback.push_error(b"tool: launch failed");
        state.last_status = 1;
        return None;
    }
    Some(InstallJob::new(rc as u32))
}

// argv as the tool sees it: argv[0] is the command name, then each argument,
// NUL-separated, which is what the kernel splits into the process argv.
fn argv_blob(args: &[&[u8]]) -> Vec<u8> {
    let mut blob = Vec::new();
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            blob.push(0);
        }
        blob.extend_from_slice(a);
    }
    blob
}
