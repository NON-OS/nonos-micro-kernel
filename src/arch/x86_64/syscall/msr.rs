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

pub const IA32_EFER: u32 = 0xC0000080;
pub const IA32_STAR: u32 = 0xC0000081;
pub const IA32_LSTAR: u32 = 0xC0000082;
pub const IA32_CSTAR: u32 = 0xC0000083;
pub const IA32_FMASK: u32 = 0xC0000084;
pub const IA32_FS_BASE: u32 = 0xC0000100;
pub const IA32_GS_BASE: u32 = 0xC0000101;
pub const IA32_KERNEL_GS_BASE: u32 = 0xC0000102;

pub const EFER_SCE: u64 = 1 << 0;
pub const EFER_LME: u64 = 1 << 8;
pub const EFER_LMA: u64 = 1 << 10;
pub const EFER_NXE: u64 = 1 << 11;

pub const RFLAGS_IF: u64 = 1 << 9;
pub const RFLAGS_TF: u64 = 1 << 8;
pub const RFLAGS_DF: u64 = 1 << 10;
pub const RFLAGS_AC: u64 = 1 << 18;

#[inline]
pub fn read_msr(msr_addr: u32) -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!(
            "rdmsr",
            in("ecx") msr_addr,
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
        ((hi as u64) << 32) | (lo as u64)
    }
}

#[inline]
pub fn write_msr(msr_addr: u32, value: u64) {
    unsafe {
        let lo = value as u32;
        let hi = (value >> 32) as u32;
        core::arch::asm!(
            "wrmsr",
            in("ecx") msr_addr,
            in("eax") lo,
            in("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
    }
}

// Encode STAR for x86_64 SYSCALL/SYSRET.
//
// Layout per Intel SDM Vol 2B: STAR[31:0] is reserved in 64-bit mode;
// STAR[47:32] is the SYSCALL selector base — the CPU loads CS from
// `STAR[47:32]` and SS from `STAR[47:32] + 8`; STAR[63:48] is the
// SYSRET selector base — the CPU loads SS from `STAR[63:48] + 8` and
// CS from `STAR[63:48] + 16`, with the low two bits of each forced to
// RPL=3. The kernel CS/SS pair and the user CS/SS pair must each sit
// at 8-byte-adjacent slots in the GDT, with the user pair laid out as
// (USER_SS, USER_CS) immediately above STAR[63:48].
//
// Caller passes the **raw** GDT selectors (RPL=0). With our GDT:
// kernel_code_raw=0x08, kernel_data_raw=0x10, user_data_raw=0x18,
// user_code_raw=0x20. STAR[63:48] is computed as user_data_raw - 8
// so that:
//   SYSRET SS = (user_data_raw - 8) + 8 | 3 = user_data_raw | 3
//   SYSRET CS = (user_data_raw - 8) + 16 | 3 = user_data_raw + 8 | 3
// which spells USER_DATA (0x1B) and USER_CODE (0x23).
pub fn setup_star(kernel_code_raw: u16, user_data_raw: u16) -> Result<(), &'static str> {
    if kernel_code_raw & 0x7 != 0 {
        return Err("kernel CS selector is not RPL0 aligned");
    }
    if user_data_raw & 0x7 != 0 {
        return Err("user data selector is not RPL0 aligned");
    }
    if user_data_raw < 8 {
        return Err("user data selector is below SYSRET base");
    }

    let sysret_base = (user_data_raw - 8) as u64;
    let syscall_field = (kernel_code_raw as u64) << 32;
    let sysret_field = sysret_base << 48;
    write_msr(IA32_STAR, syscall_field | sysret_field);
    Ok(())
}

pub fn setup_lstar(entry_point: u64) {
    write_msr(IA32_LSTAR, entry_point);
}

pub fn setup_fmask() {
    let mask = RFLAGS_IF | RFLAGS_TF | RFLAGS_DF | RFLAGS_AC;
    write_msr(IA32_FMASK, mask);
}

pub fn enable_sce() {
    let mut efer = read_msr(IA32_EFER);
    efer |= EFER_SCE;
    write_msr(IA32_EFER, efer);
}

pub fn is_sce_enabled() -> bool {
    let efer = read_msr(IA32_EFER);
    (efer & EFER_SCE) != 0
}
