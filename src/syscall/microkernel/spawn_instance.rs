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

//! Request another window instance of an embedded app capsule named by the
//! caller. The SpawnWindow capability is enforced by the syscall cap table
//! before this runs. The spawn itself is heavy (address space, ELF load,
//! stacks) and must not run inline in the caller's syscall context, which
//! corrupts the caller. So this only maps the name to an app and queues the
//! request; init performs the spawn in its own context and the window
//! appears a tick later. See `userspace::init::instance_spawn`.

use super::errnos::{ERRNO_BUSY, ERRNO_INVAL, ERRNO_NOENT};
use crate::usercopy::{read_user_bytes, validate_user_read};

// Handle names are short; cap the copy so a bad length can never over-read.
const MAX_NAME: usize = 48;

pub fn sys_spawn_instance(name_ptr: u64, name_len: u64) -> i64 {
    let n = name_len as usize;
    if n == 0 || n > MAX_NAME {
        return ERRNO_INVAL;
    }
    if validate_user_read(name_ptr, n).is_err() {
        return ERRNO_INVAL;
    }
    let bytes = match read_user_bytes(name_ptr, n) {
        Ok(b) => b,
        Err(_) => return ERRNO_INVAL,
    };
    let name = match core::str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => return ERRNO_INVAL,
    };
    let result = queue_by_name(name);
    if result >= 0 {
        boost_init_for_drain();
    }
    // Land every request in the boot log so the on-demand path is observable.
    crate::sys::serial::print(b"[SPAWN-INSTANCE] queued ");
    crate::sys::serial::print(name.as_bytes());
    crate::sys::serial::print(if result >= 0 { b" -> ok\n" } else { b" -> fail\n" });
    result
}

// The queued window spawn is drained by init, which runs at Priority::Low so an
// idle desktop leaves its cycles to the apps. A busy-yielding app with a fetch
// in flight can then starve a low-priority init off a single CPU, and the drain
// never runs, so the second window never opens. This syscall runs in the
// scheduled caller (the shell), so lift init to Normal here: init cannot boost
// itself once starved, but the click that needs the window can. Init drops back
// to Low from its own loop once the queue empties. Init is pid 1, the first
// process the kernel creates, before any capsule.
fn boost_init_for_drain() {
    const INIT_PID: u32 = 1;
    if let Some(pcb) = crate::process::core::PROCESS_TABLE.find_by_pid(INIT_PID) {
        *pcb.priority.lock() = crate::process::core::Priority::Normal;
    }
}

// Map a handle to an app that declares instance endpoints, and queue it.
// An unknown handle is rejected; a full queue asks the caller to retry.
fn queue_by_name(name: &str) -> i64 {
    use crate::userspace::init::{request_instance, PendingApp};
    let app = match name {
        "app.terminal" => PendingApp::Terminal,
        "app.browser" => PendingApp::Browser,
        "app.text_editor" => PendingApp::TextEditor,
        "app.settings" => PendingApp::Settings,
        "app.calculator" => PendingApp::Calculator,
        "app.clock" => PendingApp::Clock,
        "app.about" => PendingApp::About,
        "app.snake" => PendingApp::Snake,
        "app.nonos_wallet" => PendingApp::WalletNonos,
        "app.file_manager" => PendingApp::FileManager,
        "app.process_manager" => PendingApp::ProcessManager,
        "app.audio_player" => PendingApp::AudioPlayer,
        "app.video_player" => PendingApp::VideoPlayer,
        _ => return ERRNO_NOENT,
    };
    if request_instance(app) {
        0
    } else {
        ERRNO_BUSY
    }
}
