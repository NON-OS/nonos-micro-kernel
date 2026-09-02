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

/// Builtins worth reaching for without typing them. The palette runs the verb
/// as written, so every entry here must be a word the dispatcher accepts.
pub const VERBS: [(&str, &str); 12] = [
    ("help", "the builtins"),
    ("clear", "empty the scrollback"),
    ("ls", "list the working directory"),
    ("pwd", "print the working directory"),
    ("history", "everything run here"),
    ("jobs", "background work"),
    ("theme", "switch the palette"),
    ("version", "build and manifest"),
    ("capsules", "what is running"),
    ("whoami", "the owning identity"),
    ("motd", "the banner"),
    ("about", "this window"),
];
