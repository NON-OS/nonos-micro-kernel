// NONOS Operating System (AGPL-3.0-or-later)
// The real write-authority rule (the pure half; the per-request MkProcStat
// lookup stays in the driver, where the syscall exists).
#[path = "../../../../capsule_driver_virtio_blk/src/server/acl/rule.rs"]
pub mod rule;
