# capsule_wallpaper_catalog

## Role

`capsule_wallpaper_catalog` is the wallpaper asset catalog. It runs as a
CPL=3 capsule and serves the built-in wallpaper images to the desktop
shell and wallpaper client: it answers how many wallpapers exist, their
slugs and sizes, and streams each image back in bounded chunks. The assets
are embedded in the capsule, so it owns the catalog and needs no storage
device.

```text
desktop shell / wallpaper client
        |
        | OP_GET_COUNT / OP_GET_SLUG / OP_GET_SIZE / OP_GET_CHUNK
        v
capsule_wallpaper_catalog -- embedded wallpaper assets
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x19
```

The capsule serves callers with `MkIpcRecvFrom` plus `MkIpcSendToPid` and
terminates only through `MkExit`. It requests no hardware grants.

## Interface contract

| Operation | Input | Output |
|---|---|---|
| `OP_GET_COUNT` | none | number of wallpapers |
| `OP_GET_SLUG` | index | wallpaper slug |
| `OP_GET_SIZE` | index | byte length of the image |
| `OP_GET_CHUNK` | index, offset | bounded slice of the image |

Unknown operations reply `E_BAD_OP`. Malformed bodies reply `E_INVAL`.
An out-of-range index or offset is rejected rather than clamped.

## Authority

The capsule serves catalog data over IPC only. It has no PCI, MMIO, IRQ,
DMA, PIO, filesystem, network, display, or focus-routing authority.

## Privacy and persistence

The catalog is static and embedded; the capsule holds no per-caller state
and writes nothing to disk.
