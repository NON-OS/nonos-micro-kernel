// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Smoke-only diagnostics for `MkDebug` usercopy failures.

use crate::arch::x86_64::diag::print_hex_u64;
use crate::arch::x86_64::paging::read_cr3;
use crate::memory::layout::DIRECTMAP_BASE;
use crate::memory::paging::constants::{PTE_ADDR_MASK, PTE_HUGE_PAGE, PTE_PRESENT};
use crate::sys::serial::{print, println};
use crate::usercopy::UsercopyError;

pub(in crate::syscall::microkernel) fn validate_fail(
    user_ptr: u64,
    len: usize,
    err: UsercopyError,
) {
    let cr3 = read_cr3() & PTE_ADDR_MASK;
    let idx = indices(user_ptr);

    print(b"[MkDebug-DIAG] validate fail ptr=");
    print_hex_u64(user_ptr);
    print(b" len=");
    print_hex_u64(len as u64);
    print(b" err=");
    print(err_tag(err));
    println(b"");

    emit_indices(cr3, idx);
    emit_walk(cr3, idx);
}

pub(in crate::syscall::microkernel) fn copy_fail() {
    println(b"[MkDebug-DIAG] copy_from_user fail");
}

fn indices(va: u64) -> [u64; 4] {
    [(va >> 39) & 0x1FF, (va >> 30) & 0x1FF, (va >> 21) & 0x1FF, (va >> 12) & 0x1FF]
}

fn emit_indices(cr3: u64, idx: [u64; 4]) {
    print(b"[MkDebug-DIAG] cr3=");
    print_hex_u64(cr3);
    for (label, value) in
        [(b" i4=", idx[0]), (b" i3=", idx[1]), (b" i2=", idx[2]), (b" i1=", idx[3])]
    {
        print(label);
        print_hex_u64(value);
    }
    println(b"");
}

fn emit_walk(root: u64, idx: [u64; 4]) {
    let e4 = emit_pte(b"PML4E", root, idx[0]);
    if e4 & PTE_PRESENT == 0 {
        return;
    }
    let e3 = emit_pte(b"PDPTE", e4 & PTE_ADDR_MASK, idx[1]);
    if e3 & PTE_PRESENT == 0 || e3 & PTE_HUGE_PAGE != 0 {
        return;
    }
    let e2 = emit_pte(b"PDE", e3 & PTE_ADDR_MASK, idx[2]);
    if e2 & PTE_PRESENT == 0 || e2 & PTE_HUGE_PAGE != 0 {
        return;
    }
    emit_pte(b"PTE", e2 & PTE_ADDR_MASK, idx[3]);
}

fn emit_pte(label: &[u8], table: u64, index: u64) -> u64 {
    let value = read_pte(table, index);
    print(b"[MkDebug-DIAG] ");
    print(label);
    print(b"=");
    print_hex_u64(value);
    println(b"");
    value
}

fn read_pte(table_phys: u64, index: u64) -> u64 {
    let p = (DIRECTMAP_BASE + table_phys + index * 8) as *const u64;
    // SAFETY: table_phys came from live CR3 or a present upper-level
    // entry. index is a page-table index in 0..512.
    unsafe { core::ptr::read_volatile(p) }
}

fn err_tag(e: UsercopyError) -> &'static [u8] {
    match e {
        UsercopyError::NullPointer => b"NullPointer",
        UsercopyError::InvalidAddress => b"InvalidAddress",
        UsercopyError::AddressOverflow => b"AddressOverflow",
        UsercopyError::MisalignedAddress => b"MisalignedAddress",
        UsercopyError::PageNotMapped => b"PageNotMapped",
        UsercopyError::PageNotUser => b"PageNotUser",
        UsercopyError::PageNotWritable => b"PageNotWritable",
        UsercopyError::PageFault => b"PageFault",
        UsercopyError::NoProcessContext => b"NoProcessContext",
        UsercopyError::SizeTooLarge => b"SizeTooLarge",
        UsercopyError::InvalidUtf8 => b"InvalidUtf8",
        UsercopyError::PageTableCorrupt => b"PageTableCorrupt",
    }
}
