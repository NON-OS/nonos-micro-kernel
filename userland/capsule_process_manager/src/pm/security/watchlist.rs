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

// Services that must stay resident for the session to be trustworthy: if one
// was seen running and then vanishes, that is a real event worth flagging. This
// is deliberately narrower than the kill-guard list in `critical` (no `login`,
// which exits normally, and no `process_manager`, which is this capsule).
const SERVICES: &[&[u8]] = &[
    b"init",
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
];

pub const SERVICE_COUNT: usize = SERVICES.len();

// Index of a watched service by name, so presence can be tracked as a bitmask
// across refreshes rather than by rescanning names.
pub fn watched_index(name: &[u8]) -> Option<usize> {
    SERVICES.iter().position(|&s| s == name)
}

pub fn service_name(idx: usize) -> &'static [u8] {
    SERVICES.get(idx).copied().unwrap_or(b"")
}
