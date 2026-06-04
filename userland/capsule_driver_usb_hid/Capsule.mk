# usb_hid — USB HID class driver. Consumes descriptor bytes and
# HID boot reports supplied by the USB host-controller service.
# It owns class parsing and event normalization only; xHCI owns
# hardware transfer mechanics.

CAPSULE_SLUG             := driver-usb-hid
CAPSULE_HANDLE           := driver.usb_hid0
CAPSULE_DOMAIN           := systems.nonos
CAPSULE_DIR              := userland/capsule_driver_usb_hid
CAPSULE_BIN_NAME         := driver_usb_hid
CAPSULE_FEATURE          := nonos-capsule-driver-usb-hid
CAPSULE_NAMESPACE        := systems.nonos.driver.usb_hid0
CAPSULE_SERVICE_ENDPOINT := service:4222:driver.usb_hid0
CAPSULE_REPLY_ENDPOINT   := reply:4223:endpoint.4294967314
CAPSULE_REQUIRED_CAPS    := 0x200019

include nonos-mk/capsule.mk
