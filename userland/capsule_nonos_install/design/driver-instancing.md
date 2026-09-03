# Per-device driver instancing

The write step needs a target disk that is not the live medium, served
by its own driver instance, so that write authority lands on exactly
one device and the store the boot chain trusts stays out of reach.
This note records how instancing should work, built on machinery the
kernel already has.

## What already exists

`kernel_core/process_spawn/capsule_spawn/instance` spawns an extra
instance of an already-embedded, attested capsule. Every instance
reuses the same signed ELF, certificate, manifest and attestation
trailer; only the service and reply endpoints differ, and each
per-instance endpoint must be declared in the signed manifest. The
verified spawn path re-checks the declaration before registering, so
an instance cannot invent an endpoint that enrollment never saw. App
capsules use this today to open extra compositor windows.

Driver instancing is the same shape with two additions: the instance
must be told which device it owns, and something with authority must
ask for the spawn.

## Device assignment

The boot instance of a block driver claims its device by discovery,
first match wins. An extra instance must never re-run that race: two
drivers claiming one device is a correctness bug, and a driver
claiming the wrong disk while holding write authority is how an
installer eats the wrong target.

The assignment travels through spawn args, which the kernel already
delivers (`SYS_ARGS`). The spawner passes the PCI address of the
assigned device and the instance's endpoint name; the driver's setup
path claims exactly that address or exits with a refusal on its
serial tag. No argument means boot behavior, discovery, so the boot
spawn plan does not change.

The hardware broker enforces the other half: a device already granted
to one pid is not granted to a second. That check exists; instancing
makes it load-bearing, so the proofs should cover it.

## Endpoint naming

The enrolled manifest for `driver_virtio_blk` (and later the AHCI and
NVMe drivers) grows a small fixed table of instance endpoints:
`driver.virtio_blk1` and `driver.virtio_blk2` with their ports, next
to the boot endpoint `driver.virtio_blk0`. Fixed and small on
purpose: an installer writes one target disk, not a fleet, and every
name that can ever be registered stays visible in the signed
artifact. Growing the table is an enrollment event, which is the
correct cost for growing the attack surface.

This means one keystore re-enrollment when the manifest format gains
the table. It should ride the same ceremony as the next planned
enrollment rather than forcing its own.

## Spawn authority

Nothing userspace may spawn a driver today; the spawn plan is kernel
code. That stays. The installer does not get a spawn-driver
capability; instead the enroll service (the step 4 surface, which
already requires the physical-presence consent step) offers one
operation: instance the block driver for the disk the operator chose
in the survey. The service holds the authority, the consent gates it,
and the installer merely asks. A refused consent refuses the
instance, and with it the whole write path.

The ACL mirrors the write-authority rule: the request is honored only
from a sender whose MkProcStat entry names `app.nonos_install`,
checked per request, never cached.

## Order of work

1. Manifest instance-endpoint table for the block driver, plus the
   Capsule.mk surface to declare it. Costs the enrollment noted above.
2. Args-driven device claim in the driver's setup path, refusal
   proven in the host harness (claim wrong address, expect exit).
3. The enroll-service operation behind consent, ACL-gated to the
   installer, spawning through the existing instance machinery.
4. Installer step 5 wires capacity, read-back and the write plan
   against `driver.virtio_blk1`, with the live medium's instance
   never receiving a write request at all.

Steps 1 and 2 are self-contained and provable off the installer
entirely; 3 depends on the step 4 consent surface; 4 is the payoff.
