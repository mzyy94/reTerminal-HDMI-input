# Device Tree Overlay for connecting TC358743 to CAM0 port of CM4

## Usage

Build device tree source with `make` and install binary with `sudo make install`.

Add lines below into /boot/config.txt in order to apply overlays.


```
dtparam=i2s=on
dtparam=audio=on
dtoverlay=tc358743-cam0
dtoverlay=tc358743-audio
```

