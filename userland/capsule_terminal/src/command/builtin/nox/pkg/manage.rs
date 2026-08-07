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

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::store_status;

use super::run::USAGE;
use super::{call, emit};
use crate::term::state::State;
use crate::term::util::format_u64;

// The argument is the installed slug, the same name `pkg install` reported,
// not the path the package was installed from.
pub(super) fn remove(state: &mut State, rest: &[&[u8]]) -> bool {
    let Some(&name) = rest.first() else {
        state.scrollback.push_error(USAGE);
        return false;
    };
    match call::pkg_remove(name) {
        Ok(()) => {
            let mut line = Vec::with_capacity(8 + name.len());
            line.extend_from_slice(b"removed ");
            line.extend_from_slice(name);
            state.scrollback.push_line(&line);
            true
        }
        Err(status) => {
            emit::error(state, status);
            false
        }
    }
}

// Report whether the on-device store behind the installed packages is
// writable, so a failed install can be told apart from a broken store.
pub(super) fn status(state: &mut State) -> bool {
    match store_status() {
        Ok(0) => {
            state.scrollback.push_line(b"store healthy");
            true
        }
        Ok(code) => {
            let mut num = [0u8; 24];
            let k = format_u64(code as u64, &mut num);
            let mut line = Vec::with_capacity(12 + k);
            line.extend_from_slice(b"store error ");
            line.extend_from_slice(&num[..k]);
            state.scrollback.push_error(&line);
            false
        }
        Err(_) => {
            state.scrollback.push_error(b"store error: vfs unreachable");
            false
        }
    }
}
