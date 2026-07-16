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

// The real scheduling-policy types. `SchedAttr::effective_priority` is the
// total order this crate checks; the file is self-contained, so it is included
// whole. The unused constants and structs are the real module's.
#[allow(dead_code)]
#[path = "../../../../src/process/scheduler/policy_types.rs"]
pub mod policy_types;
