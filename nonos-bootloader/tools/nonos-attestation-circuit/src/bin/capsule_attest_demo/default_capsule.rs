// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use std::path::PathBuf;

pub fn default_capsule() -> PathBuf {
    let candidates = [
        "target/userland-clippy/x86_64-nonos-user/release/terminal",
        "../../../target/userland-clippy/x86_64-nonos-user/release/terminal",
        "target/x86_64-nonos-user/release/terminal",
        "../../../target/x86_64-nonos-user/release/terminal",
    ];
    for path in candidates {
        let candidate = PathBuf::from(path);
        if candidate.is_file() {
            return candidate;
        }
    }
    PathBuf::from(candidates[0])
}
