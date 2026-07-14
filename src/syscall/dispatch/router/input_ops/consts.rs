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

use core::sync::atomic::AtomicBool;

pub(super) const EINVAL: i32 = 22;
pub(super) const EFAULT: i32 = 14;
pub(super) const EPERM: i32 = 1;
pub(super) const ENOTSUP: i32 = 95;
pub(super) const ENOMEM: i32 = 12;
pub(super) const MAX_DRAIN: usize = 64;
pub(super) const DEFAULT_WAIT_MS: u64 = 50;
pub(super) static FIRST_INPUT_DRAIN: AtomicBool = AtomicBool::new(false);
