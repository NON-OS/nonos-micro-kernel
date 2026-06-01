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

use nonos_policy_proto::Field;

use super::kernel_field::{KERNEL_PREEMPT, KIND_BOOL};
use super::raw;

pub fn on_bool_set(field: Field, value: bool) {
    if let Field::KernelPreempt = field {
        let v = [if value { 1u8 } else { 0u8 }];
        let _ = raw::submit(KERNEL_PREEMPT, KIND_BOOL, &v);
    }
}
