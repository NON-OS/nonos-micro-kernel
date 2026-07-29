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

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::arch::x86_64::__cpuid;

pub fn read_cpuinfo() -> String {
    let mut output = String::new();
    let num_cpus = crate::smp::cpu_count();
    for cpu in 0..num_cpus {
        output.push_str(&format_cpu_entry(cpu));
        output.push('\n');
    }
    output
}

fn format_cpu_entry(cpu: usize) -> String {
    let (vendor, family, model, stepping) = get_cpu_info();
    let brand = get_cpu_brand();
    let freq = get_cpu_freq();
    let cache = get_cache_size();
    format!(
        "processor\t: {}\nvendor_id\t: {}\ncpu family\t: {}\nmodel\t\t: {}\nmodel name\t: {}\nstepping\t: {}\nmicrocode\t: 0x0\ncpu MHz\t\t: {:.3}\ncache size\t: {} KB\nphysical id\t: 0\nsiblings\t: {}\ncore id\t\t: {}\ncpu cores\t: {}\napicid\t\t: {}\ninitial apicid\t: {}\nfpu\t\t: yes\nfpu_exception\t: yes\ncpuid level\t: 22\nwp\t\t: yes\nflags\t\t: {}\nbogomips\t: {:.2}\nclflush size\t: 64\ncache_alignment\t: 64\naddress sizes\t: 48 bits physical, 48 bits virtual\npower management:\n",
        cpu, vendor, family, model, brand, stepping, freq, cache, num_cpus(), cpu, num_cpus(), cpu, cpu, get_cpu_flags(), freq * 2.0
    )
}

fn get_cpu_info() -> (&'static str, u32, u32, u32) {
    let cpuid = __cpuid(1);
    let family = (cpuid.eax >> 8) & 0xf;
    let model = (cpuid.eax >> 4) & 0xf;
    let stepping = cpuid.eax & 0xf;
    ("GenuineIntel", family, model, stepping)
}

fn get_cpu_brand() -> String {
    let mut brand = [0u8; 48];
    for i in 0..3 {
        let cpuid = __cpuid(0x80000002 + i);
        let offset = i as usize * 16;
        brand[offset..offset + 4].copy_from_slice(&cpuid.eax.to_le_bytes());
        brand[offset + 4..offset + 8].copy_from_slice(&cpuid.ebx.to_le_bytes());
        brand[offset + 8..offset + 12].copy_from_slice(&cpuid.ecx.to_le_bytes());
        brand[offset + 12..offset + 16].copy_from_slice(&cpuid.edx.to_le_bytes());
    }
    String::from_utf8_lossy(&brand).trim_end_matches('\0').trim().into()
}

fn get_cpu_freq() -> f64 {
    3000.0
}
fn get_cache_size() -> u32 {
    8192
}
fn num_cpus() -> usize {
    crate::smp::cpu_count()
}
fn get_cpu_flags() -> &'static str {
    "fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology cpuid pni pclmulqdq ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch fsgsbase bmi1 avx2 bmi2 rdseed adx clflushopt sha_ni xsaveopt xsavec xgetbv1 xsaves"
}
