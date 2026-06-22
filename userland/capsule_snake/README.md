# capsule_snake

Snake app capsule contract. Manifest mask: `CAPSULE_REQUIRED_CAPS = 0x1819`.
The capsule owns only its app state and GUI surface requests through Mk IPC,
memory and graphics calls. It has no filesystem, network or hardware authority.
