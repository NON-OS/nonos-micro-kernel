// NONOS Operating System (AGPL-3.0-or-later)
// The real NVMe transfer constants. The upstream module also carries
// queue-setup constants that only the device bring-up path uses.
#[path = "../../../capsule_driver_nvme/src/nvm/constants.rs"]
#[allow(dead_code)]
mod constants;

pub use constants::{MAX_SECTORS, SECTOR_SIZE};
