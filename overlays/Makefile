all: tc358743-cam0.dtbo


tc358743-cam0.dtbo: tc358743-cam0-overlay.dts
	dtc -@ -I dts -O dtb -o tc358743-cam0.dtbo tc358743-cam0-overlay.dts 


clean:
	rm -f tc358743-cam0.dtbo


install: tc358743-cam0.dtbo
	install -m 644 tc358743-cam0.dtbo /boot/overlays

