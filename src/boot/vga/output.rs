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

use spin::Mutex;

pub const VGA_BUFFER: usize = 0xB8000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;

static VGA_LOCK: Mutex<()> = Mutex::new(());

#[inline]
pub fn visual_delay(iterations: u32) {
    for _ in 0..iterations {
        for _ in 0..100_000 {
            // Emits the part's own spin hint: `pause` on x86_64, `yield` on
            // aarch64. Asking for the intent rather than the instruction keeps
            // the busy wait polite everywhere, and needs no unsafe block.
            core::hint::spin_loop();
        }
    }
}

/// # Safety
///
/// Writes directly to VGA memory at 0xB8000.
/// Thread-safe: acquires VGA lock before writing.
pub unsafe fn write_at(row: usize, col: usize, text: &[u8], attr: u8, delay: u32) {
    if row >= VGA_HEIGHT {
        return;
    }

    for (i, &byte) in text.iter().enumerate() {
        let x = col + i;
        if x >= VGA_WIDTH {
            break;
        }

        {
            let _lock = VGA_LOCK.lock();
            let vga = VGA_BUFFER as *mut u8;
            let offset = (row * VGA_WIDTH + x) * 2;
            // SAFETY: Bounds checked above, VGA buffer is always mapped
            unsafe {
                *vga.add(offset) = byte;
                *vga.add(offset + 1) = attr;
            }
        }

        if delay > 0 {
            visual_delay(delay);
        }
    }
}

/// # Safety
///
/// Writes directly to VGA memory.
/// Thread-safe: acquires VGA lock before writing.
pub unsafe fn clear_screen(attr: u8) {
    let _lock = VGA_LOCK.lock();
    let vga = VGA_BUFFER as *mut u8;

    for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
        let offset = i * 2;
        // SAFETY: VGA buffer is always mapped at 0xB8000
        unsafe {
            *vga.add(offset) = b' ';
            *vga.add(offset + 1) = attr;
        }
    }
}

/// # Safety
///
/// Writes directly to VGA memory.
/// Thread-safe: acquires VGA lock before writing.
#[inline]
pub unsafe fn write_string(row: usize, col: usize, text: &[u8], attr: u8) {
    // SAFETY: Caller guarantees VGA buffer access is valid
    unsafe { write_at(row, col, text, attr, 0) };
}

/// # Safety
///
/// Writes directly to VGA memory.
/// Thread-safe: acquires VGA lock before writing.
pub unsafe fn write_char(row: usize, col: usize, ch: u8, attr: u8) {
    if row >= VGA_HEIGHT || col >= VGA_WIDTH {
        return;
    }

    let _lock = VGA_LOCK.lock();
    let vga = VGA_BUFFER as *mut u8;
    let offset = (row * VGA_WIDTH + col) * 2;
    // SAFETY: Bounds checked above, VGA buffer is always mapped
    unsafe {
        *vga.add(offset) = ch;
        *vga.add(offset + 1) = attr;
    }
}

/// # Safety
///
/// Reads directly from VGA memory.
/// Thread-safe: acquires VGA lock before reading.
pub unsafe fn read_char(row: usize, col: usize) -> (u8, u8) {
    if row >= VGA_HEIGHT || col >= VGA_WIDTH {
        return (0, 0);
    }

    let _lock = VGA_LOCK.lock();
    let vga = VGA_BUFFER as *const u8;
    let offset = (row * VGA_WIDTH + col) * 2;
    // SAFETY: Bounds checked above, VGA buffer is always mapped
    let ch = unsafe { *vga.add(offset) };
    let attr = unsafe { *vga.add(offset + 1) };
    (ch, attr)
}

/// # Safety
///
/// Writes directly to VGA memory.
/// Thread-safe: acquires VGA lock before writing.
pub unsafe fn fill_row(row: usize, ch: u8, attr: u8) {
    if row >= VGA_HEIGHT {
        return;
    }

    let _lock = VGA_LOCK.lock();
    let vga = VGA_BUFFER as *mut u8;
    for col in 0..VGA_WIDTH {
        let offset = (row * VGA_WIDTH + col) * 2;
        // SAFETY: Bounds checked above, VGA buffer is always mapped
        unsafe {
            *vga.add(offset) = ch;
            *vga.add(offset + 1) = attr;
        }
    }
}

/// # Safety
///
/// Writes directly to VGA memory.
/// Thread-safe: acquires VGA lock before writing.
pub unsafe fn scroll_up(lines: usize, attr: u8) {
    if lines == 0 || lines >= VGA_HEIGHT {
        // SAFETY: clear_screen acquires its own lock
        unsafe { clear_screen(attr) };
        return;
    }

    let _lock = VGA_LOCK.lock();
    let vga = VGA_BUFFER as *mut u8;

    for row in 0..(VGA_HEIGHT - lines) {
        for col in 0..VGA_WIDTH {
            let src_offset = ((row + lines) * VGA_WIDTH + col) * 2;
            let dst_offset = (row * VGA_WIDTH + col) * 2;
            // SAFETY: Bounds checked by loop limits, VGA buffer is always mapped
            unsafe {
                *vga.add(dst_offset) = *vga.add(src_offset);
                *vga.add(dst_offset + 1) = *vga.add(src_offset + 1);
            }
        }
    }

    // Clear the newly exposed rows at the bottom
    for row in (VGA_HEIGHT - lines)..VGA_HEIGHT {
        for col in 0..VGA_WIDTH {
            let offset = (row * VGA_WIDTH + col) * 2;
            // SAFETY: Bounds checked by loop limits
            unsafe {
                *vga.add(offset) = b' ';
                *vga.add(offset + 1) = attr;
            }
        }
    }
}

pub fn buffer_size() -> usize {
    VGA_WIDTH * VGA_HEIGHT * 2
}
