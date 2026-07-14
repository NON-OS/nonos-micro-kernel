# i2c_hid — HID-over-I2C class driver. Consumes bounded I2C
# transfers from driver.i2c_pci0 and owns HID descriptor discovery
# only. The PCI controller capsule owns hardware access.

CAPSULE_SLUG             := driver-i2c-hid
CAPSULE_HANDLE           := driver.i2c_hid0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_i2c_hid
CAPSULE_BIN_NAME         := driver_i2c_hid
CAPSULE_FEATURE          := nonos-capsule-driver-i2c-hid
CAPSULE_NAMESPACE        := systems.nonos.driver.i2c_hid0
CAPSULE_SERVICE_ENDPOINT := service:4232:driver.i2c_hid0
CAPSULE_REPLY_ENDPOINT   := reply:4233:endpoint.4294967319
CAPSULE_REQUIRED_CAPS    := 0x200119
CAPSULE_KERNEL_MIRROR    := src/userspace/capsule_driver_i2c_hid

include nonos-mk/capsule.mk
