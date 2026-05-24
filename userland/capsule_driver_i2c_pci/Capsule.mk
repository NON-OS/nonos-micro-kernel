# Intel LPSS I2C PCI controller capsule. Hardware-facing only:
# PCI discovery, MMIO register window, IRQ binding, controller
# identity, and timing telemetry. HID-over-I2C/input policy stays
# above this driver.

CAPSULE_SLUG             := driver-i2c-pci
CAPSULE_HANDLE           := driver.i2c_pci0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_i2c_pci
CAPSULE_BIN_NAME         := driver_i2c_pci
CAPSULE_FEATURE          := nonos-capsule-driver-i2c-pci
CAPSULE_NAMESPACE        := systems.nonos.driver.i2c_pci0
CAPSULE_SERVICE_ENDPOINT := service:4230:driver.i2c_pci0
CAPSULE_REPLY_ENDPOINT   := reply:4231:endpoint.4294967318
# IPC|Memory|Driver|DeviceEnum|Mmio|Irq = 0x78019
CAPSULE_REQUIRED_CAPS    := 0x78019
CAPSULE_KERNEL_MIRROR    := src/hardware/i2c_pci_capsule

include nonos-mk/capsule.mk

