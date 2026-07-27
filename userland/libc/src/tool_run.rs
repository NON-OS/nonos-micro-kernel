// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Run a baked, attested command-line tool by name. The tool is spawned
//! parented to the caller, so the caller drains its stdout with `mk_proc_output`
//! and feeds its stdin with `mk_proc_input`, and waits for exit with `mk_wait`.

use crate::syscall::{call_raw, N_MK_TOOL_RUN};

/// Run the baked tool named `name` (for example `b"tool.tokei"`) with `argv`, a
/// NUL-separated `name\0arg1\0...` blob (empty for none). Returns the new tool's
/// pid, or a negative errno.
pub fn mk_tool_run(name: &[u8], argv: &[u8]) -> i64 {
    call_raw(
        N_MK_TOOL_RUN,
        [
            name.as_ptr() as u64,
            name.len() as u64,
            argv.as_ptr() as u64,
            argv.len() as u64,
            0,
            0,
        ],
    )
}
