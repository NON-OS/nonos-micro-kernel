# capsule_std_proof

Standard-library proof capsule contract. Manifest mask:
`CAPSULE_REQUIRED_CAPS = 0x19`. The capsule validates userland std support
inside the Mk syscall boundary and must not request hardware, storage or
network authority.
