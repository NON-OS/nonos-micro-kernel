# RTL8821CE firmware

`rtw8821c_fw.bin` is Realtek RTL8821CE device firmware (version 24.11.0, H2C
format 12), downloaded to the chip's 8051 at init. It is NOT part of the NONOS
source and is NOT covered by the AGPL: it is a redistributable binary from the
linux-firmware project.

- Source: linux-firmware, `rtw88/rtw8821c_fw.bin`
  https://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git
- Copyright: Realtek Semiconductor Corp.
- License: redistributable per the linux-firmware `WHENCE` entry for `rtw88`
  (Realtek redistribution terms). Retain this notice with the binary.

The driver source is original NONOS code; the Realtek register values it uses
are hardware facts referenced from the GPL `rtw88` driver, reimplemented rather
than copied to keep the NONOS tree AGPL-clean.
