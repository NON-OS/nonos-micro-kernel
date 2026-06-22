# capsule_driver_bga

## Role

Parked Bochs Graphics Adapter capsule source.

## Microkernel contract

No production spawn contract is active. A promoted version must declare
`CAPSULE_REQUIRED_CAPS` and use brokered `MkDeviceList`, `MkMmioMap` and IPC
only.

## Interface contract

No stable service endpoint is exported while parked.

## Authority

Parked status means no granted runtime authority.

## Privacy and persistence

No user data is read or persisted.

## Runtime lifecycle

Not in the production spawn set.

## Failure model

Promotion must fail closed when broker claims, MMIO mapping or display setup
fails.

## Current implemented surface

Source inventory for a future brokered BGA display capsule.

## Wire format

```text
client -> parked bga service -> no production endpoint
```

## State ownership

No production surface ownership while parked.

## Operating rules

Do not add raw MMIO or framebuffer access outside broker grants.

## Release target

Brokered BGA display driver capsule with explicit manifest capabilities.

## Release evidence

Static broker-boundary audit plus hardware or emulator display proof.

## Release checklist

- Add Capsule.mk.
- Declare `CAPSULE_REQUIRED_CAPS`.
- Register signed spawn path.
- Regenerate trust artifacts.

## Explicit non-goals today

No production display backend is claimed from this capsule today.

## Verification

Static gate: `nonos-ci/run-static-checks.sh`.
