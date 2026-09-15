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

//! A process's reply inbox name, stored inline.
//!
//! It used to be `&'static str`, which is true of a capsule baked into the
//! image and false of one loaded at runtime. The runtime path met the type by
//! leaking the name, and leaking is how a name outlives the process that owns
//! it.
//!
//! A `String` would have fixed the lifetime and put an allocation on
//! `redirect_reply`, which runs on every IPC send. This is the third option:
//! the bytes live in the process control block, cost nothing to read, and go
//! away when the process does.

/// Longer than any endpoint this system names. The longest in the tree is
/// `endpoint.app.process_manager.2.reply` at 36 bytes; the cap is set well
/// above that so a longer name is refused rather than silently cut, because a
/// truncated inbox name would route replies to the wrong reader.
pub const MAX_INBOX_NAME: usize = 64;

#[derive(Clone, Copy)]
pub struct InboxName {
    bytes: [u8; MAX_INBOX_NAME],
    len: u8,
}

impl InboxName {
    /// `None` when the name does not fit, which the caller treats as a failed
    /// spawn rather than shortening it.
    pub fn new(name: &str) -> Option<Self> {
        let src = name.as_bytes();
        if src.is_empty() || src.len() > MAX_INBOX_NAME {
            return None;
        }
        let mut bytes = [0u8; MAX_INBOX_NAME];
        bytes[..src.len()].copy_from_slice(src);
        Some(InboxName { bytes, len: src.len() as u8 })
    }

    pub fn as_str(&self) -> &str {
        /*
         * The bytes came from a &str and are never edited afterwards, so they
         * are still valid UTF-8. from_utf8 is checked anyway rather than
         * asserted: this runs on the IPC path, and a kernel that would rather
         * route nothing than route wrongly costs less here than an unsafe
         * block that has to stay right forever.
         */
        core::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("")
    }
}
