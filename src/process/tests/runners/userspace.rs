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

use crate::process::tests::userspace_types_tests as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    s.add(TestCase::new("user_cs_constant", t::test_user_cs_constant));
    s.add(TestCase::new("user_ds_constant", t::test_user_ds_constant));
    s.add(TestCase::new("kernel_cs_constant", t::test_kernel_cs_constant));
    s.add(TestCase::new("kernel_ds_constant", t::test_kernel_ds_constant));
    s.add(TestCase::new("user_rflags_constant", t::test_user_rflags_constant));
    s.add(TestCase::new("user_stack_size_constant", t::test_user_stack_size_constant));
    s.add(TestCase::new("kernel_stack_size_constant", t::test_kernel_stack_size_constant));
    s.add(TestCase::new("user_stack_base_constant", t::test_user_stack_base_constant));
    s.add(TestCase::new("user_heap_start_constant", t::test_user_heap_start_constant));
    s.add(TestCase::new("user_code_start_constant", t::test_user_code_start_constant));
    s.add(TestCase::new("segment_selectors_ring_3", t::test_segment_selectors_ring_3));
    s.add(TestCase::new("segment_selectors_ring_0", t::test_segment_selectors_ring_0));
    s.add(TestCase::new("thread_state_ready", t::test_thread_state_ready));
    s.add(TestCase::new("thread_state_running", t::test_thread_state_running));
    s.add(TestCase::new("thread_state_blocked", t::test_thread_state_blocked));
    s.add(TestCase::new("thread_state_sleeping", t::test_thread_state_sleeping));
    s.add(TestCase::new("thread_state_zombie", t::test_thread_state_zombie));
    s.add(TestCase::new("thread_state_stopped", t::test_thread_state_stopped));
    s.add(TestCase::new("thread_state_equality", t::test_thread_state_equality));
    s.add(TestCase::new("thread_state_clone", t::test_thread_state_clone));
    s.add(TestCase::new("thread_state_copy", t::test_thread_state_copy));
    s.add(TestCase::new("block_reason_io", t::test_block_reason_io));
    s.add(TestCase::new("block_reason_lock", t::test_block_reason_lock));
    s.add(TestCase::new("block_reason_futex", t::test_block_reason_futex));
    s.add(TestCase::new("block_reason_wait", t::test_block_reason_wait));
    s.add(TestCase::new("block_reason_signal", t::test_block_reason_signal));
    s.add(TestCase::new("block_reason_ipc", t::test_block_reason_ipc));
    s.add(TestCase::new("block_reason_futex_equality", t::test_block_reason_futex_equality));
    s.add(TestCase::new(
        "block_reason_different_variants",
        t::test_block_reason_different_variants,
    ));
    s.add(TestCase::new("block_reason_clone", t::test_block_reason_clone));
    s.add(TestCase::new("fpu_state_default", t::test_fpu_state_default));
    s.add(TestCase::new("fpu_state_size", t::test_fpu_state_size));
    s.add(TestCase::new("fpu_state_alignment", t::test_fpu_state_alignment));
    s.add(TestCase::new("interrupt_frame_for_user_entry", t::test_interrupt_frame_for_user_entry));
    s.add(TestCase::new("interrupt_frame_clone", t::test_interrupt_frame_clone));
    s.add(TestCase::new("interrupt_frame_copy", t::test_interrupt_frame_copy));
    s.add(TestCase::new("interrupt_frame_size", t::test_interrupt_frame_size));
    s.add(TestCase::new("user_context_default", t::test_user_context_default));
    s.add(TestCase::new("user_context_size", t::test_user_context_size));
    s.add(TestCase::new("user_context_clone", t::test_user_context_clone));
    s.add(TestCase::new("user_context_copy", t::test_user_context_copy));
    s.add(TestCase::new("exec_context_fields", t::test_exec_context_fields));
    s.add(TestCase::new("exec_context_size", t::test_exec_context_size));
    s.add(TestCase::new("user_space_addresses_valid", t::test_user_space_addresses_valid));
    s.add(TestCase::new("user_space_layout_order", t::test_user_space_layout_order));
    s.add(TestCase::new("user_rflags_interrupt_enabled", t::test_user_rflags_interrupt_enabled));
    s.add(TestCase::new("user_rflags_reserved_bit_set", t::test_user_rflags_reserved_bit_set));
    s.add(TestCase::new("stack_sizes_power_of_two", t::test_stack_sizes_power_of_two));
    s.add(TestCase::new("kernel_stack_smaller_than_user", t::test_kernel_stack_smaller_than_user));
    s.add(TestCase::new("thread_state_all_variants", t::test_thread_state_all_variants));
    s.add(TestCase::new(
        "block_reason_all_simple_variants",
        t::test_block_reason_all_simple_variants,
    ));
    s.add(TestCase::new("interrupt_frame_fields", t::test_interrupt_frame_fields));
    s.add(TestCase::new("user_context_all_registers", t::test_user_context_all_registers));
    s.add(TestCase::new("segment_selector_gdt_index", t::test_segment_selector_gdt_index));
    s.add(TestCase::new("user_context_debug", t::test_user_context_debug));
    s.add(TestCase::new("interrupt_frame_debug", t::test_interrupt_frame_debug));
    s.add(TestCase::new("thread_state_debug", t::test_thread_state_debug));
    s.add(TestCase::new("block_reason_debug", t::test_block_reason_debug));
}
