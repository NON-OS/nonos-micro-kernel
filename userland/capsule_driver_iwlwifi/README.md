# capsule_driver_iwlwifi

## Role

`capsule_driver_iwlwifi` is the Intel Wi-Fi PCIe hardware capsule. It owns the
Intel wireless device claim, BAR0 mapping, interrupt grant, DMA staging area,
firmware selection, and early APM bring-up for `driver.iwlwifi0`.

```text
net stack / wifi policy
        |
        v
driver.iwlwifi0 -- brokered MMIO/IRQ/DMA --> Intel Wi-Fi PCIe device
```

The capsule is not an IP stack, socket layer, DHCP client, WPA supplicant, or
network profile manager. Those layers stay above the hardware driver.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`, and
`Dma`:

```text
CAPSULE_REQUIRED_CAPS = 0xF8019
```

The driver reaches hardware only through `MkDeviceList`, `MkDeviceClaim`,
`MkMmioMap`, `MkIrqBind`, and `MkDmaMap`. The kernel validates the signed
manifest, routes IPC, brokers grants, and revokes every grant on capsule exit.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_HEALTHCHECK` | none | status |
| `OP_DEVICE_INFO` | none | PCI id, hw revision, family |
| `OP_FIRMWARE_INFO` | none | selected firmware name and size |
| `OP_RF_STATE` | none | RF-kill / init flags |
| `OP_DMA_STATE` | none | brokered DMA staging grant metadata |
| `OP_FIRMWARE_STAGE` | none | staged firmware section counts and bytes |
| `OP_ALIVE_WAIT` | none | alive notification status and interrupt word |

Unknown operations reply `E_BAD_OP`. Non-empty bodies on fixed-width requests
reply `E_INVAL`.

## Firmware

The capsule selects a firmware blob from the Intel files already shipped under
`nonos-bootloader/firmware/intel/`. Firmware bytes are linked into the capsule
with `include_bytes!`, so no filesystem authority is required at boot. The
firmware path validates the Intel TLV header, rejects unsupported API ranges,
stages INIT, runtime, and paging sections into a brokered DMA window, and waits
for the device alive interrupt before higher Wi-Fi runtime work begins.

## Authority

The capsule may enumerate PCI devices, claim one supported Intel Wi-Fi function,
map BAR0, bind the device IRQ, and allocate one broker-owned DMA staging region.
It has no PIO authority, no filesystem authority, no network-stack authority,
and no credential authority. The driver does not own WPA state or IP addressing.

## Privacy and persistence

The capsule stores no SSIDs, passphrases, scan history, peer MAC history, DHCP
leases, or IP state. Runtime state is limited to grant ids, PCI identity,
hardware revision, RF state, DMA metadata, and the firmware blob selected for
the detected controller family.

## Runtime lifecycle

Startup discovers a supported Intel Wi-Fi PCI function, claims it, maps BAR0,
binds its IRQ, allocates a firmware staging DMA grant, requests MAC access,
waits for the APM clock-ready bit, masks and acknowledges interrupts, reads the
hardware revision, and serves IPC. Teardown is handled by process exit and
broker revocation of the device, MMIO, IRQ, and DMA grants.

## Failure model

Every setup phase rolls back prior broker grants on failure. Unsupported Intel
PCI IDs, missing BAR0, missing IRQ, failed MMIO/IRQ/DMA grants, APM clock
timeout, invalid firmware TLV layout, staging overflow, or alive timeout prevent
the affected operation from reporting success. Association and packet transport
remain outside this hardware capsule.

## Current implemented surface

- Intel Wi-Fi PCI discovery for 7265, 8265, 9260, AX200, AX210, and BE200
  families.
- Brokered device claim, MMIO map, IRQ bind, and DMA staging allocation.
- APM clock request, interrupt mask/ack setup, and hardware revision read.
- Firmware family selection backed by bundled Intel `.ucode` blobs.
- Firmware TLV section staging into brokered DMA with deterministic section
  records.
- Alive-notification polling with interrupt acknowledgement.
- IPC health, device info, firmware info, RF state, DMA state, firmware stage,
  and alive wait.
- Static gates for brokered hardware access and endpoint ownership.

## Wire format

Requests use the `NIWF` capsule header, version `1`, and the shared 20-byte
driver envelope. Replies begin with a 4-byte signed status word. All multi-byte
integers are little-endian. Fixed-width information requests must carry no body.

## State ownership

`driver.iwlwifi0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mapping, IRQ binding, DMA staging metadata, hardware revision,
RF-kill status, and selected firmware family. The Wi-Fi runtime owns scan
policy, association state, authentication, encryption keys, and handoff into
`net.l2`.

## Operating rules

- Do not place WPA, DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles or scan history.
- Firmware upload must go through bounded DMA staging and explicit command
  completion.

## Release target

The hardware chain is:

```text
driver.iwlwifi0 -> wifi control/runtime capsule -> net.l2 -> net.ip -> apps
```

The next runtime slice is firmware command-queue execution, RX/TX ring
initialization, and passive scan command submission.

## Release evidence

Release evidence requires a signed boot with a supported Intel Wi-Fi PCI
function, successful `driver.iwlwifi0` spawn, device-info and firmware-info IPC
responses, APM clock-ready observation, RF state reporting, firmware section
upload, alive notification, and a passive scan result delivered to the Wi-Fi
runtime without kernel-resident Wi-Fi policy.

## Release checklist

- Capsule builds with zero warnings.
- Static gates confirm brokered MMIO/IRQ/DMA authority and endpoint ownership.
- Kernel profile `microkernel-driver-iwlwifi` resolves signed artifacts.
- Firmware catalog selects the expected `.ucode` blob for the detected device.
- Firmware upload and alive notification pass on supported hardware.
- Passive scan path returns bounded results without persisting SSID history.

## Explicit non-goals today

This slice does not implement association, WPA/WPA2, 802.11 frame encryption,
active scanning, regulatory-domain management, packet RX/TX handoff, monitor
mode, AP mode, or roaming policy.

## Verification

- Build: `make -B nonos-mk-driver-iwlwifi`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-iwlwifi`
- Static gate: `bash nonos-ci/run-static-checks.sh`
