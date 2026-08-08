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

// The real constant definitions the Lean specifications quote. Each Lean file
// restates these as its own literal, which is only worth something if the two
// are kept in step; `constants_tests` is what keeps them.

// `Nonos.UserCopy.userEnd`, `Nonos.DemandPaging.userTop`,
// `Nonos.Isolation.userEnd`.
#[allow(dead_code)]
#[path = "../../../../src/memory/layout/constants/canonical.rs"]
pub mod canonical;

// `Nonos.DemandPaging.pageSize`.
#[allow(dead_code)]
#[path = "../../../../src/memory/paging/constants/page_sizes.rs"]
pub mod page_sizes;

// `Nonos.Ipc.maxMessageSize`.
#[allow(dead_code)]
#[path = "../../../../src/ipc/nonos_channel/limits.rs"]
pub mod ipc_limits;

// The delegation chain depth bound. No Lean file states this one yet, so the
// test below pins it on its own and says so.
#[allow(dead_code)]
#[path = "../../../../src/capabilities/chain/constants.rs"]
pub mod chain;
