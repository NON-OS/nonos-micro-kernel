# capsule_driver_rtl8821ce

## Role

`capsule_driver_rtl8821ce` is the Realtek RTL8821CE PCIe Wi-Fi hardware capsule.
It owns the Realtek wireless device claim, BAR mapping, interrupt grant, DMA
rings, firmware download, and the MAC/PHY/RF bring-up for `driver.rtl8821ce0`.

```text
net stack / wifi policy
        |
        v
driver.rtl8821ce0 -- brokered MMIO/IRQ/DMA --> Realtek RTL8821CE PCIe device
```

The 802.11 management, WPA2 four-way handshake, and key derivation run in the
shared `nonos_wifi_core` crate; IP, DHCP, DNS, and sockets stay in the upper
network capsules. This capsule drives the radio and moves frames; it does not
own network profiles or credentials.

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum`, `Mmio`, `Irq`,
`Dma`, and (as a bring-up ceiling) `Debug`:

```text
CAPSULE_REQUIRED_CAPS = 0xF8119
```

`Debug` (0x100) is the upper bound so a serial-debug kernel can report radio
bring-up progress; a hardened build grants a subset without it and the driver
still spawns. The driver reaches hardware only through `MkDeviceList`,
`MkDeviceClaim`, `MkMmioMap`, `MkIrqBind`, and `MkDmaMap`. The kernel validates
the signed manifest, routes IPC, brokers grants, and revokes every grant on
capsule exit.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_STATUS` | none | link state, PCI id, station MAC, channel |
| `OP_SCAN` | none | scan results (BSSID, SSID, channel, RSSI) |
| `OP_CONNECT` | SSID + passphrase | association + WPA2 result |
| `OP_DISCONNECT` | none | teardown result |

Unknown operations reply with an error status word. Fixed-width requests carry
no trailing body.

## Firmware

The capsule downloads the Realtek RTL8821CE firmware image into the device over
the H2C/DDMA path. Firmware bytes are linked into the capsule with
`include_bytes!`, so no filesystem authority is required at boot. The firmware
path validates the header, stages the sections into a brokered DMA window, drives
the download registers, and waits for the firmware-ready state before MAC init.

## Authority

The capsule may enumerate PCI devices, claim one supported Realtek Wi-Fi
function, map its BARs, bind the device IRQ, and allocate broker-owned DMA rings.
It has no filesystem authority, no credential authority, and no network-stack
authority. Passphrases arrive only for the duration of a connect request and are
handed to the shared supplicant; the driver keeps no profile store.

## Privacy and persistence

The capsule stores no SSIDs, passphrases, scan history, peer MAC history, DHCP
leases, or IP state. Runtime state is limited to grant ids, PCI identity, the
station MAC read from efuse, radio/link state, DMA ring metadata, the current
channel, and the session keys the supplicant installs for the active link.

## Runtime lifecycle

Startup discovers a supported Realtek Wi-Fi PCI function, claims it, maps its
BARs, binds the IRQ, allocates the TX/RX DMA rings, downloads firmware, reads the
station MAC from efuse and programs it into the MAC, runs the MAC/BB/RF init
tables, and serves IPC. A connect request scans, associates, runs the WPA2
four-way through `nonos_wifi_core`, installs the pairwise and group keys into the
hardware crypto engine, and reports the link up. Teardown is handled by process
exit and broker revocation of the device, MMIO, IRQ, and DMA grants.

## Failure model

Every setup phase rolls back prior broker grants on failure. Unsupported PCI
IDs, missing BARs, missing IRQ, failed MMIO/IRQ/DMA grants, firmware download
timeout, MAC/PHY init failure, association timeout, or a failed four-way prevent
the affected operation from reporting success. Packet transport above the link
layer stays in the upper network capsules.

## Current implemented surface

- Realtek RTL8821CE PCI discovery and brokered device claim.
- Firmware download over the H2C/DDMA path with bounded DMA staging.
- Station MAC read from efuse and programmed into the MAC id registers.
- MAC, baseband, and RF init tables with per-channel RF retune.
- TX and RX descriptor rings serviced from the device IRQ.
- Hardware security engine enable with pairwise/group key install for CCMP.
- Scan, associate, and WPA2 four-way through the shared `nonos_wifi_core`.
- IPC status, scan, connect, and disconnect.

## Wire format

Requests use the capsule header, version `1`, and the shared driver envelope.
Replies begin with a signed status word. All multi-byte integers are
little-endian.

## State ownership

`driver.rtl8821ce0` owns only hardware-facing Wi-Fi state: PCI identity, broker
grant ids, BAR mappings, IRQ binding, DMA ring metadata, station MAC, radio and
link state, current channel, and the installed session keys. The Wi-Fi core owns
scan policy, association state machine, authentication, and key derivation;
`net.l2` and above own addressing and transport.

## Operating rules

- Do not place DHCP, DNS, IP, or socket policy in this capsule.
- Do not import kernel driver or memory internals.
- Do not use inline PIO or architecture assembly.
- Do not persist Wi-Fi profiles, passphrases, or scan history.
- Firmware download must go through bounded DMA staging and explicit completion.

## Release target

The hardware chain is:

```text
driver.rtl8821ce0 -> nonos_wifi_core -> net.l2 -> net.ip -> apps
```

## Verification

- Build: `make -B nonos-mk-driver-rtl8821ce`
- Kernel profile: `cargo check --no-default-features --features
  microkernel-driver-rtl8821ce`
- Static gate: `bash nonos-ci/run-static-checks.sh`
