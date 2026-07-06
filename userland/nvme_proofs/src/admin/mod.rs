// NONOS Operating System (AGPL-3.0-or-later)
// The real NVMe identify and SMART/health parsers, which run on
// device-controlled bytes.
#[path = "../../../capsule_driver_nvme/src/admin/health/mod.rs"]
mod health;
#[path = "../../../capsule_driver_nvme/src/admin/identity.rs"]
mod identity;
#[path = "../../../capsule_driver_nvme/src/admin/namespace.rs"]
mod namespace;

pub use health::SmartHealth;
pub use identity::ControllerIdentity;
pub use namespace::NamespaceIdentity;
