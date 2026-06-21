# capsule_crypto

## Role

`capsule_crypto` is the userland cryptographic operation capsule. It exposes
the approved crypto service surface over IPC so user requests do not call
kernel-resident crypto shims.

```text
client capsule
    |
    | crypto IPC
    v
crypto -- operation dispatcher --> hash / verify / crypto response
```

## Microkernel contract

The capsule uses IPC and memory only:

- `MkIpcRecv` receives requests on `service:4102:crypto_pool`.
- `MkIpcSend` replies on `reply:4103:endpoint.4294967300`.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/security/crypto_capsule`.

The kernel keeps boot-time and TCB cryptography where required, but user-facing
crypto requests route through this capsule.

## Interface contract

| Surface | Purpose |
|---|---|
| hash | user-facing digest operations |
| verify | signature verification operations as they are promoted |
| AEAD | encryption/decryption operations through the capsule protocol |
| errno mapping | deterministic failure reporting for malformed requests |

## Authority

The manifest grants `IPC`, `Memory`, and service authority through
`CAPSULE_REQUIRED_CAPS = 0x38`. It has no driver, MMIO, IRQ, DMA, PIO,
filesystem, network, admin, or debug authority.

## Privacy and persistence

Request buffers are processed in capsule memory and replies are returned over
IPC. The capsule does not persist plaintexts, digests, signatures, or keys.
Long-lived secret storage belongs to `capsule_keyring`, not here.

## Runtime lifecycle

The capsule receives bounded crypto requests, dispatches to operation handlers,
returns the result, and retains no request material after completion.

## Failure model

Unsupported operation, malformed payload, oversized input, verification
failure, or crypto backend failure return explicit protocol errors. There is no
silent fallback to syscall-side crypto shims.

## Current implemented surface

- Owns the user-facing crypto protocol.
- Dispatches supported operations through modular server handlers.
- Is embedded, spawned, and validation-covered for the hash surface.
- Keeps user crypto off the syscall fast path inside the kernel.

## Wire format

Requests carry operation id, algorithm id where applicable, input lengths, and
payload bytes. Replies carry status and result bytes. All operation families
must define fixed bounds before they are promoted.

## State ownership

The capsule owns transient operation buffers only. `capsule_keyring` owns
long-lived key material. The kernel owns boot/TCB crypto only and does not
serve user crypto requests directly.

## Operating rules

- Bound every input and output length.
- Keep plaintext/key material transient.
- Return explicit verification and decode errors.
- Do not bypass the capsule through syscall-side crypto shims.

## Release target

The finished crypto capsule exposes the approved hash, signature, verification,
and AEAD operations through one audited IPC surface, with fixed wire layouts,
bounded input sizes, zero persistent plaintext/key storage, and validation coverage
per operation family.

## Release evidence

Release evidence is operation-family validation coverage, request-size boundary
tests, and static proof that user-facing crypto syscalls route through the
capsule client.

## Release checklist

- Hash validation passes.
- Signature verify validation passes when promoted.
- AEAD validation passes when promoted.
- Boundary tests cover oversized input and malformed payloads.
- Static gate confirms no user-facing kernel crypto shim.

## Explicit non-goals today

No filesystem key store, TLS stack, certificate database, hardware accelerator
driver, network protocol, or persistent audit log lives in this capsule.

## Verification

- Build: `make -B nonos-mk-crypto`
- Validation: `nonos-mk-crypto-hash-test`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: user-facing hash and AEAD syscall paths must route
  through the capsule client rather than kernel shims.
