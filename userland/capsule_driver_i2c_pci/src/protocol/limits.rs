// A real HID report descriptor runs several hundred bytes (Windows Precision
// Touchpad descriptors reach ~600-1000), and it is fetched in a single read.
// The read ceiling has to cover it or enumeration never completes; the kernel
// IPC channel allows up to 1 MiB, so 1 KiB here is comfortably in range.
// Writes stay tiny (a register address plus a short command).
pub const IPC_PAYLOAD_MAX: usize = 1088;
pub const TRANSFER_WRITE_MAX: usize = 64;
pub const TRANSFER_READ_MAX: usize = 1024;
