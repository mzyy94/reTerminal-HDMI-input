# Device Tree Overlay for connecting TC358743 to CAM0 port of CM4

_Originally from [Two B102 TC358743 simultaneously on CM4 - Raspberry Pi Forums](https://forums.raspberrypi.com/viewtopic.php?t=303226)_

## Usage

Build device tree source with `make` and install binary with `sudo make install`.

Add lines below into /boot/config.txt in order to apply overlays.


```
dtparam=i2s=on
dtparam=audio=on
dtoverlay=tc358743-cam0
dtoverlay=tc358743-audio
```

