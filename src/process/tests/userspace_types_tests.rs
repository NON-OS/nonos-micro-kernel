// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

use crate::process::userspace::types::{
    BlockReason, ExecContext, FpuState, InterruptFrame, ThreadState, UserContext, KERNEL_CS,
    KERNEL_DS, KERNEL_STACK_SIZE, USER_CODE_START, USER_CS, USER_DS, USER_HEAP_START, USER_RFLAGS,
    USER_STACK_BASE, USER_STACK_SIZE,
};
use crate::test::framework::TestResult;

pub(crate) fn test_user_cs_constant() -> TestResult {
    if USER_CS != 0x1B {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_ds_constant() -> TestResult {
    if USER_DS != 0x23 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_kernel_cs_constant() -> TestResult {
    if KERNEL_CS != 0x08 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_kernel_ds_constant() -> TestResult {
    if KERNEL_DS != 0x10 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_rflags_constant() -> TestResult {
    if USER_RFLAGS != 0x202 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_stack_size_constant() -> TestResult {
    if USER_STACK_SIZE != 2 * 1024 * 1024 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_kernel_stack_size_constant() -> TestResult {
    if KERNEL_STACK_SIZE != 32 * 1024 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_stack_base_constant() -> TestResult {
    if USER_STACK_BASE != 0x0000_7FFF_FFFF_0000 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_heap_start_constant() -> TestResult {
    if USER_HEAP_START != 0x0000_0001_0000_0000 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_code_start_constant() -> TestResult {
    if USER_CODE_START != 0x0000_0000_0040_0000 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_segment_selectors_ring_3() -> TestResult {
    if USER_CS & 0x3 != 3 {
        return TestResult::Fail;
    }
    if USER_DS & 0x3 != 3 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_segment_selectors_ring_0() -> TestResult {
    if KERNEL_CS & 0x3 != 0 {
        return TestResult::Fail;
    }
    if KERNEL_DS & 0x3 != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_ready() -> TestResult {
    let state = ThreadState::Ready;
    if state != ThreadState::Ready {
        return TestResult::Fail;
    }
    if state as u8 != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_running() -> TestResult {
    let state = ThreadState::Running;
    if state != ThreadState::Running {
        return TestResult::Fail;
    }
    if state as u8 != 1 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_blocked() -> TestResult {
    let state = ThreadState::Blocked;
    if state != ThreadState::Blocked {
        return TestResult::Fail;
    }
    if state as u8 != 2 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_sleeping() -> TestResult {
    let state = ThreadState::Sleeping;
    if state != ThreadState::Sleeping {
        return TestResult::Fail;
    }
    if state as u8 != 3 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_zombie() -> TestResult {
    let state = ThreadState::Zombie;
    if state != ThreadState::Zombie {
        return TestResult::Fail;
    }
    if state as u8 != 4 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_stopped() -> TestResult {
    let state = ThreadState::Stopped;
    if state != ThreadState::Stopped {
        return TestResult::Fail;
    }
    if state as u8 != 5 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_equality() -> TestResult {
    if ThreadState::Ready != ThreadState::Ready {
        return TestResult::Fail;
    }
    if ThreadState::Ready == ThreadState::Running {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_clone() -> TestResult {
    let state = ThreadState::Blocked;
    let cloned = state.clone();
    if state != cloned {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_copy() -> TestResult {
    let state = ThreadState::Running;
    let copied = state;
    if state != copied {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_io() -> TestResult {
    let reason = BlockReason::Io;
    if reason != BlockReason::Io {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_lock() -> TestResult {
    let reason = BlockReason::Lock;
    if reason != BlockReason::Lock {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_futex() -> TestResult {
    let reason = BlockReason::Futex(0x12345678);
    if let BlockReason::Futex(addr) = reason {
        if addr != 0x12345678 {
            return TestResult::Fail;
        }
    } else {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_wait() -> TestResult {
    let reason = BlockReason::Wait;
    if reason != BlockReason::Wait {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_signal() -> TestResult {
    let reason = BlockReason::Signal;
    if reason != BlockReason::Signal {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_ipc() -> TestResult {
    let reason = BlockReason::Ipc;
    if reason != BlockReason::Ipc {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_futex_equality() -> TestResult {
    let r1 = BlockReason::Futex(100);
    let r2 = BlockReason::Futex(100);
    let r3 = BlockReason::Futex(200);
    if r1 != r2 {
        return TestResult::Fail;
    }
    if r1 == r3 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_different_variants() -> TestResult {
    if BlockReason::Io == BlockReason::Lock {
        return TestResult::Fail;
    }
    if BlockReason::Wait == BlockReason::Signal {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_clone() -> TestResult {
    let reason = BlockReason::Futex(42);
    let cloned = reason.clone();
    if reason != cloned {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_fpu_state_default() -> TestResult {
    let fpu = FpuState::default();
    if fpu.data.len() != 1024 {
        return TestResult::Fail;
    }
    if !fpu.data.iter().all(|&b| b == 0) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_fpu_state_size() -> TestResult {
    if core::mem::size_of::<FpuState>() != 1024 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_fpu_state_alignment() -> TestResult {
    if core::mem::align_of::<FpuState>() != 64 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_for_user_entry() -> TestResult {
    let frame = InterruptFrame::for_user_entry(0x401000, 0x7FFFFF000);
    if frame.rip != 0x401000 {
        return TestResult::Fail;
    }
    if frame.cs != USER_CS as u64 {
        return TestResult::Fail;
    }
    if frame.rflags != USER_RFLAGS {
        return TestResult::Fail;
    }
    if frame.rsp != 0x7FFFFF000 {
        return TestResult::Fail;
    }
    if frame.ss != USER_DS as u64 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_clone() -> TestResult {
    let frame1 = InterruptFrame::for_user_entry(0x401000, 0x7FFFFF000);
    let frame2 = frame1.clone();
    if frame1.rip != frame2.rip {
        return TestResult::Fail;
    }
    if frame1.cs != frame2.cs {
        return TestResult::Fail;
    }
    if frame1.rflags != frame2.rflags {
        return TestResult::Fail;
    }
    if frame1.rsp != frame2.rsp {
        return TestResult::Fail;
    }
    if frame1.ss != frame2.ss {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_copy() -> TestResult {
    let frame1 = InterruptFrame::for_user_entry(0x500000, 0x8000000);
    let frame2 = frame1;
    if frame1.rip != frame2.rip {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_size() -> TestResult {
    if core::mem::size_of::<InterruptFrame>() != 40 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_default() -> TestResult {
    let ctx = UserContext::default();
    if ctx.rax != 0 {
        return TestResult::Fail;
    }
    if ctx.rbx != 0 {
        return TestResult::Fail;
    }
    if ctx.rcx != 0 {
        return TestResult::Fail;
    }
    if ctx.rdx != 0 {
        return TestResult::Fail;
    }
    if ctx.rsi != 0 {
        return TestResult::Fail;
    }
    if ctx.rdi != 0 {
        return TestResult::Fail;
    }
    if ctx.rbp != 0 {
        return TestResult::Fail;
    }
    if ctx.rsp != 0 {
        return TestResult::Fail;
    }
    if ctx.r8 != 0 {
        return TestResult::Fail;
    }
    if ctx.r9 != 0 {
        return TestResult::Fail;
    }
    if ctx.r10 != 0 {
        return TestResult::Fail;
    }
    if ctx.r11 != 0 {
        return TestResult::Fail;
    }
    if ctx.r12 != 0 {
        return TestResult::Fail;
    }
    if ctx.r13 != 0 {
        return TestResult::Fail;
    }
    if ctx.r14 != 0 {
        return TestResult::Fail;
    }
    if ctx.r15 != 0 {
        return TestResult::Fail;
    }
    if ctx.rip != 0 {
        return TestResult::Fail;
    }
    if ctx.cs != 0 {
        return TestResult::Fail;
    }
    if ctx.rflags != 0 {
        return TestResult::Fail;
    }
    if ctx.ss != 0 {
        return TestResult::Fail;
    }
    if ctx.fs_base != 0 {
        return TestResult::Fail;
    }
    if ctx.gs_base != 0 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_size() -> TestResult {
    if core::mem::size_of::<UserContext>() != 22 * 8 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_clone() -> TestResult {
    let mut ctx1 = UserContext::default();
    ctx1.rax = 42;
    ctx1.rip = 0x401000;
    ctx1.rsp = 0x7FFFFF000;
    let ctx2 = ctx1.clone();
    if ctx1.rax != ctx2.rax {
        return TestResult::Fail;
    }
    if ctx1.rip != ctx2.rip {
        return TestResult::Fail;
    }
    if ctx1.rsp != ctx2.rsp {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_copy() -> TestResult {
    let mut ctx1 = UserContext::default();
    ctx1.rdi = 100;
    let ctx2 = ctx1;
    if ctx1.rdi != ctx2.rdi {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_exec_context_fields() -> TestResult {
    let ctx = ExecContext {
        entry: 0x401000,
        stack: 0x7FFFFF000,
        pid: 1234,
        tid: 5678,
        cr3: 0x100000,
        argc: 3,
        argv: 0x7FFFFFE00,
        envp: 0x7FFFFFE80,
    };
    if ctx.entry != 0x401000 {
        return TestResult::Fail;
    }
    if ctx.stack != 0x7FFFFF000 {
        return TestResult::Fail;
    }
    if ctx.pid != 1234 {
        return TestResult::Fail;
    }
    if ctx.tid != 5678 {
        return TestResult::Fail;
    }
    if ctx.cr3 != 0x100000 {
        return TestResult::Fail;
    }
    if ctx.argc != 3 {
        return TestResult::Fail;
    }
    if ctx.argv != 0x7FFFFFE00 {
        return TestResult::Fail;
    }
    if ctx.envp != 0x7FFFFFE80 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_exec_context_size() -> TestResult {
    if core::mem::size_of::<ExecContext>() != 8 * 8 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_space_addresses_valid() -> TestResult {
    if !(USER_STACK_BASE < 0x0000_8000_0000_0000) {
        return TestResult::Fail;
    }
    if !(USER_HEAP_START < 0x0000_8000_0000_0000) {
        return TestResult::Fail;
    }
    if !(USER_CODE_START < 0x0000_8000_0000_0000) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_space_layout_order() -> TestResult {
    if !(USER_CODE_START < USER_HEAP_START) {
        return TestResult::Fail;
    }
    if !(USER_HEAP_START < USER_STACK_BASE) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_rflags_interrupt_enabled() -> TestResult {
    if !((USER_RFLAGS & 0x200) != 0) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_rflags_reserved_bit_set() -> TestResult {
    if !((USER_RFLAGS & 0x2) != 0) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_stack_sizes_power_of_two() -> TestResult {
    if !((USER_STACK_SIZE & (USER_STACK_SIZE - 1)) == 0) {
        return TestResult::Fail;
    }
    if !((KERNEL_STACK_SIZE & (KERNEL_STACK_SIZE - 1)) == 0) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_kernel_stack_smaller_than_user() -> TestResult {
    if !(KERNEL_STACK_SIZE < USER_STACK_SIZE) {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_all_variants() -> TestResult {
    let states = [
        ThreadState::Ready,
        ThreadState::Running,
        ThreadState::Blocked,
        ThreadState::Sleeping,
        ThreadState::Zombie,
        ThreadState::Stopped,
    ];
    for (i, state) in states.iter().enumerate() {
        if *state as u8 != i as u8 {
            return TestResult::Fail;
        }
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_all_simple_variants() -> TestResult {
    let reasons = [
        BlockReason::Io,
        BlockReason::Lock,
        BlockReason::Wait,
        BlockReason::Signal,
        BlockReason::Ipc,
    ];
    for reason in reasons {
        let cloned = reason.clone();
        if reason != cloned {
            return TestResult::Fail;
        }
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_fields() -> TestResult {
    let frame =
        InterruptFrame { rip: 0xDEADBEEF, cs: 0x1B, rflags: 0x202, rsp: 0xCAFEBABE, ss: 0x23 };
    if frame.rip != 0xDEADBEEF {
        return TestResult::Fail;
    }
    if frame.cs != 0x1B {
        return TestResult::Fail;
    }
    if frame.rflags != 0x202 {
        return TestResult::Fail;
    }
    if frame.rsp != 0xCAFEBABE {
        return TestResult::Fail;
    }
    if frame.ss != 0x23 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_all_registers() -> TestResult {
    let ctx = UserContext {
        r15: 15,
        r14: 14,
        r13: 13,
        r12: 12,
        r11: 11,
        r10: 10,
        r9: 9,
        r8: 8,
        rdi: 7,
        rsi: 6,
        rbp: 5,
        rbx: 4,
        rdx: 3,
        rcx: 2,
        rax: 1,
        rip: 0x400000,
        cs: 0x1B,
        rflags: 0x202,
        rsp: 0x7FFFFFF0,
        ss: 0x23,
        fs_base: 0x100,
        gs_base: 0x200,
    };
    if ctx.r15 != 15 {
        return TestResult::Fail;
    }
    if ctx.r8 != 8 {
        return TestResult::Fail;
    }
    if ctx.rax != 1 {
        return TestResult::Fail;
    }
    if ctx.fs_base != 0x100 {
        return TestResult::Fail;
    }
    if ctx.gs_base != 0x200 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_segment_selector_gdt_index() -> TestResult {
    if (USER_CS >> 3) & 0x1FFF != 3 {
        return TestResult::Fail;
    }
    if (USER_DS >> 3) & 0x1FFF != 4 {
        return TestResult::Fail;
    }
    if (KERNEL_CS >> 3) & 0x1FFF != 1 {
        return TestResult::Fail;
    }
    if (KERNEL_DS >> 3) & 0x1FFF != 2 {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_user_context_debug() -> TestResult {
    let ctx = UserContext::default();
    let debug_str = alloc::format!("{:?}", ctx);
    if !debug_str.contains("UserContext") {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_interrupt_frame_debug() -> TestResult {
    let frame = InterruptFrame::for_user_entry(0x401000, 0x7FFFFF000);
    let debug_str = alloc::format!("{:?}", frame);
    if !debug_str.contains("InterruptFrame") {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_thread_state_debug() -> TestResult {
    let state = ThreadState::Running;
    let debug_str = alloc::format!("{:?}", state);
    if !debug_str.contains("Running") {
        return TestResult::Fail;
    }
    TestResult::Pass
}

pub(crate) fn test_block_reason_debug() -> TestResult {
    let reason = BlockReason::Futex(0x1000);
    let debug_str = alloc::format!("{:?}", reason);
    if !debug_str.contains("Futex") {
        return TestResult::Fail;
    }
    TestResult::Pass
}
