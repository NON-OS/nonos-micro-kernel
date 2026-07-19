// NONOS Operating System (AGPL-3.0-or-later)
// The real MSI-X interrupt-bind validator from src/hardware/broker/irq,
// included via #[path].
#[path = "../../../../../src/hardware/broker/irq/types.rs"]
pub mod types;

#[path = "../../../../../src/hardware/broker/irq/validate.rs"]
pub mod validate;
