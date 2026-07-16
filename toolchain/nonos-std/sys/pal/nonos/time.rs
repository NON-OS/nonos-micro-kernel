// NONOS std PAL: Instant reads the kernel monotonic clock (MMON, TSC base,
// no NTP) so it never runs backwards; SystemTime reads the NTP-corrected
// wall clock (MTMS, Unix ms seeded from the RTC at boot). Millisecond
// resolution.

use crate::time::Duration;

const fn tag4(b: &[u8; 4]) -> i64 {
    (b[0] as i64) | ((b[1] as i64) << 8) | ((b[2] as i64) << 16) | ((b[3] as i64) << 24)
}

const N_MK_TIME_MILLIS: i64 = tag4(b"MTMS");
const N_MK_TIME_MONOTONIC: i64 = tag4(b"MMON");

fn read_ms(tag: i64) -> u64 {
    let r: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inout("rax") tag => r,
            out("rcx") _,
            out("r11") _,
        );
    }
    if r < 0 { 0 } else { r as u64 }
}

// Wall clock (Unix ms, NTP-corrected) backs SystemTime.
fn wall_ms() -> u64 {
    read_ms(N_MK_TIME_MILLIS)
}

// Monotonic ms (TSC base, no NTP) backs Instant so it never runs backwards.
fn mono_ms() -> u64 {
    read_ms(N_MK_TIME_MONOTONIC)
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

impl Instant {
    pub fn now() -> Instant {
        Instant(Duration::from_millis(mono_ms()))
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(*other)?))
    }
}

impl SystemTime {
    pub const MAX: SystemTime = SystemTime(Duration::MAX);
    pub const MIN: SystemTime = SystemTime(Duration::ZERO);

    pub fn now() -> SystemTime {
        SystemTime(Duration::from_millis(wall_ms()))
    }

    // Wall-clock stamp from a unix-millisecond count, the form the VFS store
    // records file times in. UNIX_EPOCH is a zero Duration, so the offset is
    // the millisecond count itself.
    pub fn from_unix_millis(millis: u64) -> SystemTime {
        SystemTime(Duration::from_millis(millis))
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
