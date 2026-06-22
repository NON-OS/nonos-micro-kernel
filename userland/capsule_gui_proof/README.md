# capsule_gui_proof

GUI proof capsule contract. This capsule is parked unless a `Capsule.mk`
declares a service endpoint and `CAPSULE_REQUIRED_CAPS`. It must not own
hardware; GUI authority must flow through Mk graphics and IPC surfaces only.
