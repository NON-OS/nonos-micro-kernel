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

use crate::syscall::tests::caps as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    s.add(TestCase::new(
        "caps::can_exit_requires_core_exec",
        t::test_capability_token_can_exit_requires_core_exec,
    ));
    s.add(TestCase::new(
        "caps::can_exit_without_core_exec",
        t::test_capability_token_can_exit_without_core_exec,
    ));
    s.add(TestCase::new("caps::can_getpid", t::test_capability_token_can_getpid));
    s.add(TestCase::new("caps::can_fork", t::test_capability_token_can_fork));
    s.add(TestCase::new("caps::can_exec", t::test_capability_token_can_exec));
    s.add(TestCase::new("caps::can_wait", t::test_capability_token_can_wait));
    s.add(TestCase::new("caps::can_signal", t::test_capability_token_can_signal));
    s.add(TestCase::new("caps::can_read", t::test_capability_token_can_read));
    s.add(TestCase::new("caps::can_read_without_io", t::test_capability_token_can_read_without_io));
    s.add(TestCase::new("caps::can_write", t::test_capability_token_can_write));
    s.add(TestCase::new("caps::can_open_files", t::test_capability_token_can_open_files));
    s.add(TestCase::new("caps::can_close_files", t::test_capability_token_can_close_files));
    s.add(TestCase::new("caps::can_stat", t::test_capability_token_can_stat));
    s.add(TestCase::new("caps::can_seek", t::test_capability_token_can_seek));
    s.add(TestCase::new("caps::can_modify_dirs", t::test_capability_token_can_modify_dirs));
    s.add(TestCase::new("caps::can_unlink", t::test_capability_token_can_unlink));
    s.add(TestCase::new("caps::can_allocate_memory", t::test_capability_token_can_allocate_memory));
    s.add(TestCase::new(
        "caps::can_deallocate_memory",
        t::test_capability_token_can_deallocate_memory,
    ));
    s.add(TestCase::new("caps::can_network", t::test_capability_token_can_network));
    s.add(TestCase::new("caps::can_ipc", t::test_capability_token_can_ipc));
    s.add(TestCase::new("caps::can_crypto", t::test_capability_token_can_crypto));
    s.add(TestCase::new("caps::can_hardware", t::test_capability_token_can_hardware));
    s.add(TestCase::new("caps::can_debug", t::test_capability_token_can_debug));
    s.add(TestCase::new("caps::can_admin", t::test_capability_token_can_admin));
    s.add(TestCase::new(
        "caps::empty_cannot_do_anything",
        t::test_capability_token_empty_cannot_do_anything,
    ));
    s.add(TestCase::new(
        "caps::multiple_capabilities",
        t::test_capability_token_multiple_capabilities,
    ));
    s.add(TestCase::new("caps::all_capabilities", t::test_capability_token_all_capabilities));
}
