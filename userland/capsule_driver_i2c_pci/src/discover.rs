use nonos_libc::{mk_device_list, Bar, DeviceRecord};

use crate::constants::{device_info, INTEL_VENDOR_ID};

const MAX_DEVICES: usize = 128;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub irq_line: u8,
    pub bar0_size: u64,
    pub pci_device: u16,
    pub clock_hz: u32,
    pub family: &'static str,
}

pub fn find_controller() -> Option<Found> {
    let mut buf = [empty_record(); MAX_DEVICES];
    let n = mk_device_list(0, buf.as_mut_ptr(), MAX_DEVICES as u64);
    if n <= 0 {
        return None;
    }
    for r in &buf[..core::cmp::min(n as usize, MAX_DEVICES)] {
        if r.vendor != INTEL_VENDOR_ID || r.irq_pin == 0 || r.irq_line == 0xFF {
            continue;
        }
        let Some((family, clock_hz)) = device_info(r.device) else { continue };
        let bar0 = r.bars[0];
        if r.bar_count != 0 && bar0.size != 0 {
            return Some(Found {
                device_id: r.device_id,
                irq_line: r.irq_line,
                bar0_size: bar0.size,
                pci_device: r.device,
                clock_hz,
                family,
            });
        }
    }
    None
}

fn empty_record() -> DeviceRecord {
    DeviceRecord {
        device_id: 0,
        bus_kind: 0,
        _pad0: [0; 3],
        class: 0,
        vendor: 0,
        device: 0,
        flags: 0,
        bar_count: 0,
        irq_line: 0xFF,
        irq_pin: 0,
        _pad1: [0; 1],
        irq_source: 0,
        bars: [Bar { base: 0, size: 0, kind: 0, flags: 0, _pad: [0; 6] }; 6],
        virtio_present: 0,
        virtio_common_bar: 0,
        virtio_notify_bar: 0,
        virtio_device_bar: 0,
        virtio_common_off: 0,
        virtio_notify_off: 0,
        virtio_device_off: 0,
        virtio_isr_off: 0,
        virtio_notify_mult: 0,
    }
}
