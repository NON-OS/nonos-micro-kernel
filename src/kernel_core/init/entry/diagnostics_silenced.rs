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

//! On-screen ACPI touchpad boot-log line, kept for bring-up on new hardware.
//! Silenced in the production boot path; the whole helper set is retained here
//! commented out so it can be re-enabled without rewriting it.

/* bring-up diagnostic, silenced
// Render what the ACPI DSDT/SSDT enumeration found for the touchpad onto the
// on-screen boot log, as the last kernel line before userspace so it stays
// visible in a photo. On real hardware this prints the touchpad's true I2C
// slave address, interrupt pin and HID descriptor register: the ground truth
// the blind-probe path could only guess at.
fn log_acpi_touchpad_onscreen() {
    let devices = crate::arch::x86_64::acpi::aml::enumerate_i2c_hid();
    let Some(d) = devices.first() else {
        boot_log::stage("ACPI-HID", "no i2c-hid device in ACPI DSDT/SSDT");
        return;
    };
    let mut buf = [0u8; 96];
    let mut n = 0;
    n += put(&mut buf[n..], b"addr=0x");
    n += put_hex(&mut buf[n..], d.slave_addr as u64);
    n += put(&mut buf[n..], b" gpio=");
    n += put_dec(&mut buf[n..], d.gpio_pin as u64);
    n += put(&mut buf[n..], b" reg=0x");
    n += put_hex(&mut buf[n..], d.hid_desc_reg as u64);
    n += put(&mut buf[n..], b" hid=");
    for &c in d.hid.iter() {
        if c == 0 {
            break;
        }
        if n < buf.len() {
            buf[n] = c;
            n += 1;
        }
    }
    n += put(&mut buf[n..], b" ctrl=");
    for &c in d.controller.iter() {
        if c == 0 {
            break;
        }
        if n < buf.len() {
            buf[n] = c;
            n += 1;
        }
    }
    let msg = core::str::from_utf8(&buf[..n]).unwrap_or("acpi-hid: format error");
    boot_log::stage("ACPI-HID", msg);
}

fn put(out: &mut [u8], s: &[u8]) -> usize {
    let k = s.len().min(out.len());
    out[..k].copy_from_slice(&s[..k]);
    k
}

fn put_dec(out: &mut [u8], mut v: u64) -> usize {
    let mut tmp = [0u8; 20];
    let mut k = 0;
    loop {
        tmp[k] = b'0' + (v % 10) as u8;
        v /= 10;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k.min(out.len()) {
        out[i] = tmp[k - 1 - i];
    }
    k.min(out.len())
}

fn put_hex(out: &mut [u8], mut v: u64) -> usize {
    let mut tmp = [0u8; 16];
    let mut k = 0;
    loop {
        let d = (v & 0xF) as u8;
        tmp[k] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
        v >>= 4;
        k += 1;
        if v == 0 {
            break;
        }
    }
    for i in 0..k.min(out.len()) {
        out[i] = tmp[k - 1 - i];
    }
    k.min(out.len())
}
*/
