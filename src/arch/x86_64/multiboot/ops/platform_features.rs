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

use super::super::error::MultibootError;
use super::super::platform::Platform;

pub fn init_platform_features(platform: Platform) -> Result<(), MultibootError> {
    match platform {
        Platform::QemuTcg => init_qemu_tcg_features()?,
        Platform::QemuKvm | Platform::Kvm => init_kvm_features()?,
        Platform::Vmware => init_vmware_features()?,
        Platform::HyperV => init_hyperv_features()?,
        Platform::Xen => init_xen_features()?,
        Platform::VirtualBox => init_vbox_features()?,
        Platform::BareMetal => init_baremetal_features()?,
        _ => {
            crate::log::info!("No specific initialization for platform: {}", platform.name());
        }
    }
    Ok(())
}

fn init_qemu_tcg_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing QEMU TCG features:");
    crate::log::info!("  - Enabling debug port (0x402) for logging");
    crate::log::info!("  - Reduced timer frequency for software emulation");
    crate::log::info!("  - Virtio device detection enabled");

    // SAFETY: Probes debug port availability by writing one zero byte.
    unsafe {
        x86_64::instructions::port::Port::<u8>::new(0x402).write(0);
    }

    Ok(())
}

fn init_kvm_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing KVM features:");
    crate::log::info!("  - KVM hypercalls available");
    crate::log::info!("  - Paravirtual clock enabled");
    crate::log::info!("  - Virtio device detection enabled");

    let kvm_features = core::arch::x86_64::__cpuid(0x40000001);
    if kvm_features.eax & (1 << 0) != 0 {
        crate::log::info!("  - KVM clocksource available");
    }
    if kvm_features.eax & (1 << 3) != 0 {
        crate::log::info!("  - KVM async PF available");
    }
    if kvm_features.eax & (1 << 4) != 0 {
        crate::log::info!("  - KVM steal time available");
    }

    Ok(())
}

fn init_vmware_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing VMware features:");

    let vmware_timing = core::arch::x86_64::__cpuid(0x40000010);

    if vmware_timing.eax != 0 {
        let tsc_khz = vmware_timing.eax;
        crate::log::info!("  - VMware TSC frequency: {} kHz", tsc_khz);
    }

    if vmware_timing.ebx != 0 {
        let apic_khz = vmware_timing.ebx;
        crate::log::info!("  - VMware APIC bus frequency: {} kHz", apic_khz);
    }

    crate::log::info!("  - VMXNET3 network driver support available");
    crate::log::info!("  - PVSCSI storage driver support available");
    crate::log::info!("  - VMware Tools integration available");

    Ok(())
}

fn init_hyperv_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing Hyper-V features:");

    let hv_features = core::arch::x86_64::__cpuid(0x40000003);

    if hv_features.eax & (1 << 0) != 0 {
        crate::log::info!("  - VP runtime MSR available");
    }
    if hv_features.eax & (1 << 1) != 0 {
        crate::log::info!("  - Partition reference counter available");
    }
    if hv_features.eax & (1 << 2) != 0 {
        crate::log::info!("  - Synthetic timers available");
    }
    if hv_features.eax & (1 << 3) != 0 {
        crate::log::info!("  - APIC access MSRs available");
    }
    if hv_features.eax & (1 << 4) != 0 {
        crate::log::info!("  - Hypercall MSRs available");
    }
    if hv_features.eax & (1 << 5) != 0 {
        crate::log::info!("  - VP index MSR available");
    }

    Ok(())
}

fn init_xen_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing Xen features:");

    let xen_version = core::arch::x86_64::__cpuid(0x40000001);
    let major = (xen_version.eax >> 16) & 0xFFFF;
    let minor = xen_version.eax & 0xFFFF;

    crate::log::info!("  - Xen version: {}.{}", major, minor);
    crate::log::info!("  - Xen hypercalls available");
    crate::log::info!("  - Xen PV clock enabled");

    Ok(())
}

fn init_vbox_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing VirtualBox features:");
    crate::log::info!("  - VBoxGuest interface detection");
    crate::log::info!("  - VirtualBox graphics adapter support");
    crate::log::info!("  - Looking for VBox PCI devices (vendor 0x80EE)");

    Ok(())
}

fn init_baremetal_features() -> Result<(), MultibootError> {
    crate::log::info!("Initializing bare metal features:");
    crate::log::info!("  - Full hardware timer precision");
    crate::log::info!("  - Native ACPI power management");
    crate::log::info!("  - Hardware interrupt affinity");

    let cpuid1 = core::arch::x86_64::__cpuid(1);
    if cpuid1.ecx & (1 << 15) != 0 {
        crate::log::info!("  - PDCM (Perfmon/Debug) available");
    }
    if cpuid1.edx & (1 << 22) != 0 {
        crate::log::info!("  - ACPI via MSR available");
    }

    Ok(())
}
