# NONOS bootloader

The NONOS bootloader is a UEFI loader for the signed kernel image and the
transparent boot-attestation path. It verifies the kernel hash and signature,
initializes boot attestation state, and fails closed when production freshness
or machine identity material is missing.

The current ZK path is transparent enrolled-secret attestation. There is no
Groth16 circuit, trusted setup, proving key, verifying key registry or ceremony
in the active bootloader path.

Boot attestation uses two proof modes:

- static embedded proof for kernel-image binding
- runtime sidecar proof for challenge-bound production boot

The runtime sidecar context binds:

```text
kernel_hash[32] || boot_nonce[32] || machine_id[32] || timestamp_be[8]
```

Production mode requires a valid runtime sidecar when a pending challenge is
present. Invalid sidecars fail closed instead of falling back to an embedded
proof. Machine identity and boot nonce initialization also fail closed in
production.

The host-side transparent tooling lives in:

```text
nonos-bootloader/tools/nonos-attestation-circuit
nonos-bootloader/tools/embed-zk-proof
```

The top-level `make help` output is the source of truth for current build and
verification entry points.
