# home-watcher

## Preparation

Create the following udev rule under ` /etc/udev/rules.d`.

```
ACTION=="add", ATTRS{idVendor}=="0590", ATTRS{idProduct}=="00d4", RUN+="/bin/sh -c 'echo 0590 00d4 > /sys/bus/usb-serial/drivers/generic/new_id'", MODE="0666" SYMLINK+="2JCIE-BU"
```

Then, reload udev rules.

```shell
$ sudo udevadm control --reload
```
