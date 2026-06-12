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

use crate::process::tests::fd_types_tests as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    s.add(TestCase::new("fd_type_variants", t::fd_type_variants));
    s.add(TestCase::new(
        "fd_type_not_equal_different_variants",
        t::fd_type_not_equal_different_variants,
    ));
    s.add(TestCase::new("fd_entry_new", t::fd_entry_new));
    s.add(TestCase::new("fd_entry_with_pipe_read", t::fd_entry_with_pipe_read));
    s.add(TestCase::new("fd_entry_with_pipe_write", t::fd_entry_with_pipe_write));
    s.add(TestCase::new("fd_entry_is_cloexec", t::fd_entry_is_cloexec));
    s.add(TestCase::new("fd_cloexec_constant", t::fd_cloexec_constant));
    s.add(TestCase::new("max_process_fds_constant", t::max_process_fds_constant));
    s.add(TestCase::new("stdio_fds_constant", t::stdio_fds_constant));
    s.add(TestCase::new("fd_entry_clone", t::fd_entry_clone));
    s.add(TestCase::new("fd_table_stats_default", t::fd_table_stats_default));
    s.add(TestCase::new("process_fd_table_new", t::process_fd_table_new));
    s.add(TestCase::new("process_fd_table_allocate", t::process_fd_table_allocate));
    s.add(TestCase::new("process_fd_table_allocate_at", t::process_fd_table_allocate_at));
    s.add(TestCase::new("process_fd_table_allocate_min", t::process_fd_table_allocate_min));
    s.add(TestCase::new("process_fd_table_get", t::process_fd_table_get));
    s.add(TestCase::new("process_fd_table_remove", t::process_fd_table_remove));
    s.add(TestCase::new("process_fd_table_is_valid", t::process_fd_table_is_valid));
    s.add(TestCase::new("process_fd_table_get_type", t::process_fd_table_get_type));
    s.add(TestCase::new("process_fd_table_close_all", t::process_fd_table_close_all));
    s.add(TestCase::new("process_fd_table_cloexec", t::process_fd_table_cloexec));
    s.add(TestCase::new("process_fd_table_status_flags", t::process_fd_table_status_flags));
    s.add(TestCase::new("process_fd_table_dup", t::process_fd_table_dup));
    s.add(TestCase::new("process_fd_table_dup2", t::process_fd_table_dup2));
    s.add(TestCase::new("process_fd_table_dup2_same_fd", t::process_fd_table_dup2_same_fd));
    s.add(TestCase::new(
        "process_fd_table_dup2_replaces_existing",
        t::process_fd_table_dup2_replaces_existing,
    ));
    s.add(TestCase::new("process_fd_table_close_cloexec", t::process_fd_table_close_cloexec));
    s.add(TestCase::new("process_fd_table_fork", t::process_fd_table_fork));
    s.add(TestCase::new("process_fd_table_stats", t::process_fd_table_stats));
    s.add(TestCase::new(
        "process_fd_table_allocate_at_invalid",
        t::process_fd_table_allocate_at_invalid,
    ));
    s.add(TestCase::new(
        "process_fd_table_allocate_min_invalid",
        t::process_fd_table_allocate_min_invalid,
    ));
    s.add(TestCase::new(
        "process_fd_table_dup2_invalid_new_fd",
        t::process_fd_table_dup2_invalid_new_fd,
    ));
    s.add(TestCase::new("process_fd_table_dup_nonexistent", t::process_fd_table_dup_nonexistent));
    s.add(TestCase::new(
        "process_fd_table_set_cloexec_nonexistent",
        t::process_fd_table_set_cloexec_nonexistent,
    ));
    s.add(TestCase::new(
        "process_fd_table_set_status_flags_nonexistent",
        t::process_fd_table_set_status_flags_nonexistent,
    ));
}
