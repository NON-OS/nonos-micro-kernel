# capsule_entropy

## Role

`capsule_entropy` is the userland entropy service. It receives entropy material
from approved sources, maintains the runtime pool, and serves entropy requests
over IPC.

```text
entropy source / client
    |
    | entropy IPC
    v
entropy -- runtime pool --> response bytes
```

## Microkernel contract

The capsule is reached through IPC:

- `MkIpcRecv` receives service requests on `service:4100:entropy_pool`.
- `MkIpcSend` replies on `reply:4101:endpoint.4294967299`.
- `MkExit` terminates on fatal startup failure.
- The kernel mirror is `src/security/entropy_capsule`.

The kernel does not serve user entropy directly from in-kernel policy paths.
It starts and routes the capsule, then leaves entropy service behavior in
userland.

## Interface contract

| Surface | Purpose |
|---|---|
| entropy request | return bounded entropy bytes to callers |
| reseed path | accept approved source material when wired |
| health state | fail closed when source quality is insufficient |

## Authority

The manifest grants `IPC`, `Memory`, and service authority through
`CAPSULE_REQUIRED_CAPS = 0x38`. It has no driver, MMIO, IRQ, DMA, PIO,
filesystem, network, admin, or debug authority.

## Privacy and persistence

Entropy pool state is runtime-only. The capsule does not persist samples or
write the pool to disk. Request payloads and response bytes are transient IPC
data.

## Runtime lifecycle

The capsule starts its pool, serves bounded entropy requests, updates runtime
pool state, and exits without writing pool material to persistent storage.

## Failure model

Bad request sizes, insufficient pool state, and source failure return protocol
errors. The service must not fabricate entropy to satisfy callers.

## Current implemented surface

- Owns a userland entropy pool.
- Exposes request/response protocol handlers.
- Is embedded, spawned, and validation-covered through the kernel mirror.
- Removes user-facing entropy policy from the kernel.

## Wire format

Requests carry the requested byte count and operation id. Replies carry status
and entropy bytes up to the protocol maximum. Source-ingest messages, when
wired, must name the source class and byte count explicitly.

## State ownership

The capsule owns pool state, source-health state, and request accounting.
Hardware drivers own collection. The kernel owns no user entropy pool.

## Operating rules

- Reject oversized requests.
- Fail closed when source quality is insufficient.
- Do not write pool bytes to persistent storage.
- Do not fabricate entropy to satisfy a request.

## Release target

The finished entropy capsule accepts approved entropy sources, tracks source
health, reseeds on policy, returns bounded responses, fails closed when source
quality is insufficient, and is covered by validation checks for request, reseed, and
failure paths. Hardware collection remains in driver capsules.

## Release evidence

Release evidence is entropy validation coverage for request, reseed, exhausted-source
failure, and syscall routing proof that user random requests do not call kernel
RNG shims.

## Release checklist

- Request and reseed validation passes.
- Exhausted-source failure is covered.
- Static gate confirms user random requests route through the capsule.
- No persistent seed path exists without a dedicated design.

## Explicit non-goals today

No hardware driver, persistent seed file, TPM integration, remote entropy
source, cryptographic API, or key generation policy lives here.

## Verification

- Build: `make -B nonos-mk-entropy`
- Validation: `nonos-mk-entropy-test`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: user-facing random requests must route through this
  capsule path, not a kernel RNG shim.
