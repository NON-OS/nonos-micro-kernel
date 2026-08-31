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

use nonos_cap::{CAP_CORE_EXEC, CAP_MEMORY};

/// What every program needs to exist at all: the right to run, and memory to
/// run in.
///
/// This is the floor, not a default set of conveniences. An app that declares
/// nothing gets exactly this and can do nothing else — it cannot draw, reach
/// the network, or read a file. Everything beyond it has to be asked for by
/// name, which is the point: what an app can do should be visible in its
/// source, in its manifest, and to the person installing it.
pub const BASE: u64 = CAP_CORE_EXEC | CAP_MEMORY;
