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

// Processes that hold the system or desktop up: ending one strands the session
// or the kernel. The monitor still allows it (the authority is real), but arms
// an extra confirmation so it is never a single stray keypress.
const CRITICAL: &[&[u8]] = &[
    b"init",
    b"login",
    b"keyring",
    b"entropy_pool",
    b"crypto_pool",
    b"policy",
    b"input_router",
    b"vfs_pool",
    b"net.core",
    b"compositor",
    b"wm",
    b"desktop_shell",
    b"process_manager",
];

pub fn is_critical(name: &[u8]) -> bool {
    CRITICAL.contains(&name)
}
