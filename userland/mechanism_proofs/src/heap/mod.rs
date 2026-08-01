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

// The real allocator every capsule linking `nonos_userland_libc` frees
// through. `nonos_alloc` carries a copy of the same wrapper for the capsules
// that link it instead; the test here covers this one, and the two are kept
// identical deliberately.
#[allow(dead_code)]
#[path = "../../../../userland/libc/src/heap/zero_on_free.rs"]
pub mod zero_on_free;
