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

use super::platform::{
    get_stack_pointer, random64_or_counter, read_cycle_counter, read_pit_counter,
    read_rtc_timestamp, KEYGEN_COUNTER,
};
use crate::crypto::blake3_hash;
use crate::crypto::util::rng;
use core::sync::atomic::Ordering;

pub fn generate_secure_key() -> [u8; 32] {
    use crate::drivers::virtio_rng;
    let mut entropy_pool = [0u8; 256];
    let mut offset = 0;
    let mut has_true_randomness = false;
    if virtio_rng::is_available() {
        let mut virt_buf = [0u8; 64];
        if virtio_rng::fill_random(&mut virt_buf).is_ok() {
            entropy_pool[offset..offset + 64].copy_from_slice(&virt_buf);
            offset += 64;
            has_true_randomness = true;
            for b in virt_buf.iter_mut() {
                unsafe { core::ptr::write_volatile(b, 0) };
            }
        }
    }
    let tsc_start = read_cycle_counter();
    for _ in 0..8 {
        let val = random64_or_counter();
        entropy_pool[offset..offset + 8].copy_from_slice(&val.to_le_bytes());
        offset += 8;
    }
    let tsc1 = read_cycle_counter();
    entropy_pool[offset..offset + 8].copy_from_slice(&tsc1.to_le_bytes());
    offset += 8;
    let stack_addr = get_stack_pointer();
    let heap_entropy = entropy_pool.as_ptr() as u64;
    let initial_jitter = tsc1.wrapping_sub(tsc_start);
    let mixed = stack_addr ^ heap_entropy ^ tsc1 ^ initial_jitter.wrapping_mul(0x517cc1b727220a95);
    entropy_pool[offset..offset + 8].copy_from_slice(&mixed.to_le_bytes());
    offset += 8;
    for i in 0..4 {
        let pit_delay = read_pit_counter();
        let delay_loops = ((pit_delay as u32) & 0xFF).wrapping_add((i as u32 + 1) * 50);
        for _ in 0..delay_loops {
            core::hint::spin_loop();
        }
        let val = random64_or_counter();
        entropy_pool[offset..offset + 8].copy_from_slice(&val.to_le_bytes());
        offset += 8;
    }
    for i in 0..8 {
        let pit_val = read_pit_counter();
        entropy_pool[offset..offset + 2].copy_from_slice(&pit_val.to_le_bytes());
        offset += 2;
        for _ in 0..((pit_val & 0x3F) as u32 + (i as u32 + 1) * 10) {
            core::hint::spin_loop();
        }
    }
    let tsc2 = read_cycle_counter();
    entropy_pool[offset..offset + 8].copy_from_slice(&tsc2.to_le_bytes());
    offset += 8;
    let jitter = tsc2.wrapping_sub(tsc1);
    let mixed_jitter = jitter.wrapping_mul(0x9E3779B97F4A7C15).rotate_right((jitter & 31) as u32);
    entropy_pool[offset..offset + 8].copy_from_slice(&mixed_jitter.to_le_bytes());
    offset += 8;
    let rtc_time = read_rtc_timestamp();
    entropy_pool[offset..offset + 8].copy_from_slice(&rtc_time.to_le_bytes());
    offset += 8;
    let kernel_ms = crate::time::timestamp_millis();
    let pit_now = read_pit_counter() as u64;
    let time_entropy = kernel_ms ^ (pit_now << 48) ^ (pit_now << 32) ^ (pit_now << 16);
    entropy_pool[offset..offset + 8].copy_from_slice(&time_entropy.to_le_bytes());
    offset += 8;
    let mut rng_bytes = [0u8; 32];
    rng::fill_random_bytes(&mut rng_bytes);
    entropy_pool[offset..offset + 32].copy_from_slice(&rng_bytes);
    offset += 32;
    let pit_for_delay = read_pit_counter();
    for _ in 0..(pit_for_delay & 0x7F) as u32 {
        core::hint::spin_loop();
    }
    let tsc3 = read_cycle_counter();
    entropy_pool[offset..offset + 8].copy_from_slice(&tsc3.to_le_bytes());
    offset += 8;
    let jitter2 = tsc3.wrapping_sub(tsc2);
    let total_jitter = tsc3.wrapping_sub(tsc_start);
    let combined = jitter2 ^ total_jitter.rotate_left(17) ^ jitter.rotate_right(23);
    entropy_pool[offset..offset + 8].copy_from_slice(&combined.to_le_bytes());
    if !has_true_randomness {
        crate::log_warn!(
            "crypto: No virtio-rng! Add -device virtio-rng-pci to QEMU for unique wallets"
        );
    }
    offset += 8;
    for i in 0..4 {
        let pit_d = read_pit_counter();
        for _ in 0..(pit_d & 0x1F) as u32 + i * 8 {
            core::hint::spin_loop();
        }
        let val = random64_or_counter();
        entropy_pool[offset..offset + 8].copy_from_slice(&val.to_le_bytes());
        offset += 8;
    }
    let counter = KEYGEN_COUNTER.fetch_add(0xA3B7_C1D5_E9F2_4680, Ordering::SeqCst);
    let tsc_final = read_cycle_counter();
    let counter_mixed = counter ^ tsc_final ^ tsc_final.wrapping_sub(tsc_start);
    entropy_pool[offset..offset + 8].copy_from_slice(&counter_mixed.to_le_bytes());
    let key = blake3_hash(&entropy_pool);
    for byte in entropy_pool.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    for byte in rng_bytes.iter_mut() {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
    key
}
