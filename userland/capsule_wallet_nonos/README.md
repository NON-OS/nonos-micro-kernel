# capsule_wallet_nonos

NONOS wallet capsule contract. Manifest mask: `CAPSULE_REQUIRED_CAPS = 0x1819`.
The capsule owns wallet UI state and must keep key handling inside its signed
capsule boundary. It has no raw device authority; all interaction goes through
Mk IPC, memory and GUI surfaces.
