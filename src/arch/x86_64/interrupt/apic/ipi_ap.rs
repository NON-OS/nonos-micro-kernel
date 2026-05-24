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

use super::constants::*;
use super::ipi_basic::wait_icr_idle;
use super::mmio::mmio_w32;
use super::state::*;
use core::sync::atomic::Ordering;

pub fn start_ap(apic_id: u32, start_page: u8) {
    icr_send(apic_id, ICR_DELIV_INIT | ICR_LEVEL_ASSERT | ICR_TRIG_EDGE, 0);
    delay_us_via_tsc(10);
    icr_send(apic_id, ICR_DELIV_INIT | ICR_LEVEL_DEASSERT | ICR_TRIG_EDGE, 0);
    delay_us_via_tsc(200);
    icr_send(apic_id, ICR_DELIV_SIPI | ICR_TRIG_EDGE, start_page);
    delay_us_via_tsc(200);
    icr_send(apic_id, ICR_DELIV_SIPI | ICR_TRIG_EDGE, start_page);
}

fn icr_send(apic_id: u32, mode: u64, vec: u8) {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, (apic_id as u64) << 32 | mode | vec as u64);
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, apic_id << 24);
        mmio_w32(LAPIC_ICR_LOW, mode as u32 | vec as u32);
    }
}

fn delay_us_via_tsc(us: u32) {
    let freq = crate::sys::timer::tsc::tsc_frequency();
    if freq == 0 {
        pit_spin_us(us);
        return;
    }
    let ticks = (freq as u128 * us as u128 / 1_000_000u128) as u64;
    let start = crate::sys::timer::tsc::rdtsc();
    while crate::sys::timer::tsc::rdtsc().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
}

fn pit_spin_us(us: u32) {
    let iters = (us as u64).saturating_mul(2_000);
    for _ in 0..iters {
        // SAFETY: I/O port 0x80 is the legacy POST debug port; reading it
        // is a well-known ~1us bus delay used as a PIT-substitute spin.
        unsafe {
            let _: u8;
            core::arch::asm!(
                "in al, dx",
                in("dx") 0x80u16,
                out("al") _,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}
