pub fn fill(base: u64, width: u32, height: u32, stride: u32, argb: u32) {
    for y in 0..height {
        for x in 0..width {
            let cell = (base + (y as u64 * stride as u64 + x as u64 * 4)) as *mut u32;
            // SAFETY: base is a mapped ARGB8888 surface of stride*height bytes
            // exclusively owned by this capsule until release; the loop stays
            // within (width, height) so every cell falls inside the mapping.
            unsafe { core::ptr::write_volatile(cell, argb) };
        }
    }
}
