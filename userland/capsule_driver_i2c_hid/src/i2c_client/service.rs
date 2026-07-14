use nonos_libc::{mk_service_lookup, mk_yield};

// The i2c controller driver registers driver.i2c_pci0 during its own setup,
// which can finish after this capsule starts. A single lookup then races the
// controller's registration and loses, and the driver would exit believing
// there is no controller. Retry with yields so a slightly later registration
// is still picked up, bounded so a genuinely absent controller does not spin
// forever.
const LOOKUP_ATTEMPTS: u32 = 2000;
const YIELDS_BETWEEN: u32 = 64;

pub fn resolve() -> Option<(u32, u32)> {
    let name = b"driver.i2c_pci0";
    let mut attempt = 0;
    loop {
        let mut port = 0u32;
        let mut pid = 0u32;
        let r = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        if r >= 0 && port != 0 && pid != 0 {
            return Some((port, pid));
        }
        attempt += 1;
        if attempt >= LOOKUP_ATTEMPTS {
            return None;
        }
        for _ in 0..YIELDS_BETWEEN {
            mk_yield();
        }
    }
}
