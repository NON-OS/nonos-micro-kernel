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

use crate::syscall::tests::types as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    s.add(TestCase::new("types::syscall_result_success", t::test_syscall_result_success));
    s.add(TestCase::new("types::syscall_result_success_zero", t::test_syscall_result_success_zero));
    s.add(TestCase::new("types::syscall_result_success_max", t::test_syscall_result_success_max));
    s.add(TestCase::new(
        "types::syscall_result_success_audited",
        t::test_syscall_result_success_audited,
    ));
    s.add(TestCase::new("types::syscall_result_error", t::test_syscall_result_error));
    s.add(TestCase::new(
        "types::syscall_result_error_value_is_negated",
        t::test_syscall_result_error_value_is_negated,
    ));
    s.add(TestCase::new(
        "types::syscall_result_is_error_positive",
        t::test_syscall_result_is_error_positive,
    ));
    s.add(TestCase::new(
        "types::syscall_result_is_error_zero",
        t::test_syscall_result_is_error_zero,
    ));
    s.add(TestCase::new(
        "types::syscall_result_is_error_negative",
        t::test_syscall_result_is_error_negative,
    ));
    s.add(TestCase::new(
        "types::syscall_result_errno_none_for_success",
        t::test_syscall_result_errno_none_for_success,
    ));
    s.add(TestCase::new(
        "types::syscall_result_errno_some_for_error",
        t::test_syscall_result_errno_some_for_error,
    ));
    s.add(TestCase::new("types::errno_helper", t::test_errno_helper));
    s.add(TestCase::new("types::errno_helper_eperm", t::test_errno_helper_eperm));
    s.add(TestCase::new("types::errno_helper_enoent", t::test_errno_helper_enoent));
    s.add(TestCase::new(
        "types::syscall_result_const_success",
        t::test_syscall_result_const_success,
    ));
    s.add(TestCase::new("types::syscall_result_const_error", t::test_syscall_result_const_error));
}
