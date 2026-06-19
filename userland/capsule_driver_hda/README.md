# capsule_driver_hda

## Role

`capsule_driver_hda` is the Intel HD Audio controller capsule. It owns the PCI
HDA controller in userland and exposes controller capability, codec-presence
state, codec vendor identity, and stream descriptor layout over IPC.

This slice proves controller discovery, broker claim, BAR0 mapping, IRQ
ownership, reset release, GCAP/GCTL/STATESTS reporting, codec mask reporting,
immediate-command codec probing, and stream-descriptor offset derivation. Audio
playback and capture come later, after CORB/RIRB and stream DMA are real.

```text
driver.hda0
    |
    | broker claim + BAR0 MMIO
    v
HDA controller registers
    |
    `-- IRQ ownership for controller events
```

## Microkernel contract

The capsule uses only Mk/broker calls:

- `MkDeviceList` locates Intel HDA controller records.
- `MkDeviceClaim` gives this process exclusive controller ownership.
- `MkMmioMap` maps the BAR0 register window.
- `MkIrqBind`, `MkIrqPoll`, and `MkIrqAck` own controller interrupts.
- `MkIpcRecv` and `MkIpcSend` serve `driver.hda0` on
  `service:4218:driver.hda0`.

The kernel keeps scheduling, isolation, capability checks, and grant teardown.
It does not mix audio, route streams, parse codec widgets, or hold user audio.

## Interface contract

| Operation | Meaning | Reply payload |
|---|---|---|
| `OP_HEALTHCHECK` | server liveness | status word |
| `OP_CONTROLLER_INFO` | GCAP/GCTL/STATESTS snapshot | 28-byte controller record |
| `OP_CODEC_MASK` | detected codec slots | 8-byte mask payload |
| `OP_STREAM_LAYOUT` | GCAP-derived stream descriptor offsets | count plus 8-byte entries |
| `OP_CODEC_LIST` | immediate-command codec vendor ids | count plus 8-byte entries |

## Authority

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, and
`Irq` (`CAPSULE_REQUIRED_CAPS = 0x78018`). There is no `Dma` grant until the
capsule programs CORB/RIRB and stream buffers.

```text
allowed:   HDA controller claim, BAR0 registers, IRQ, IPC
forbidden: DMA streams, filesystem, mixer policy, microphone policy, admin
```

## Privacy and persistence

This slice never records or plays audio. It reports controller registers,
codec-presence bits, codec vendor/device ids, and descriptor offsets only. No
samples are persisted, no microphone input is captured, and no runtime state
survives process exit.

## Runtime lifecycle

The capsule claims the HDA controller, maps BAR0, binds IRQ, releases reset,
reads controller state, records codec-presence bits, probes codec vendor ids
through the immediate command registers, derives stream descriptor offsets from
GCAP, and serves IPC. Teardown unwinds IRQ, MMIO, and claim grants.

## Failure model

Broker setup failure aborts startup. A controller that does not leave reset or
reports unusable state is surfaced as a setup error. Runtime requests never
program streams in this slice.

## Current implemented surface

- Claims the HDA controller through `MkDeviceClaim`.
- Maps BAR0 through `MkMmioMap`.
- Binds the controller IRQ.
- Releases controller reset and reads GCAP/GCTL/STATESTS.
- Reports codec-presence state over IPC.
- Probes codec vendor/device ids with `Get Parameter(Vendor ID)`.
- Reports input, output, and bidirectional stream descriptor offsets over IPC.
- Fails closed on broker or setup failure.

## Wire format

Requests use the `NHDA` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies start with a 4-byte status word. Controller-info
replies return a 28-byte fixed register snapshot. Codec-mask replies return an
8-byte mask payload. Stream-layout replies return a 4-byte count followed by
8-byte entries:

```text
u8 kind, u8 local_index, u16 global_index, u32 stream_descriptor_offset
```

Codec-list replies return a 4-byte count followed by 8-byte entries:

```text
u8 codec_address, u8 probe_ok, u16 vendor_id, u16 device_id, u16 reserved
```

## State ownership

The capsule owns BAR0 mapping, IRQ grant, controller reset state, GCAP/GCTL
snapshot, codec-presence mask, immediate-command probe result, and GCAP-derived
stream descriptor layout. Future stream state, CORB/RIRB rings, and BDLs belong
here too, not in the kernel.

## Operating rules

- Do not expose playback or capture until CORB/RIRB and stream DMA are real.
- Keep mixer, policy routing, and permissions outside the controller driver.
- Do not persist samples or stream state.
- Any setup failure must unwind IRQ, MMIO, and device claim.

## Release target

The finished HDA capsule is a signed audio-controller service with CORB/RIRB
verb transport, codec discovery, stream descriptor programming, BDL DMA, IRQ
completion, and playback/capture stream endpoints. Mixer, policy routing,
permissions, and user audio session control remain outside the kernel and above
the controller driver.

## Release evidence

Release requires QEMU `intel-hda` controller proof, codec verb transport validation,
stream DMA playback proof, IRQ completion proof, and teardown revocation proof.

## Release checklist

- Signed manifest and kernel mirror present.
- QEMU HDA controller probe passes.
- Codec verb round trip works through CORB/RIRB.
- Playback DMA validation produces completion interrupts.
- Teardown proof shows no leaked MMIO/IRQ/DMA grants.

## Explicit non-goals today

No CORB/RIRB, stream descriptor programming, BDL, PCM playback, PCM capture,
mixer, jack policy, volume policy, or persistent audio state is implemented
here. The only codec verb path in this slice is immediate-command
`Get Parameter(Vendor ID)` for inventory.

## Verification

- Build: `make -B nonos-mk-driver-hda`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Architecture check: HDA must remain a userland capsule using broker MMIO/IRQ
  only in this slice.
- Documentation check: the static gate requires this README and its authority,
  privacy, current surface, non-goal, and verification sections.
