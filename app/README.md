# Broadcasat termainal App

![broadcast-terminal](/docs/app.png)

_Work in progress_

## Dependencies

- Rust 1.58
- Debian Packages

```
sudo apt-get install libgtk-3-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev
```

- Vulkan driver

Install Vulkan 1.1 on Raspberry Pi OS (32bit) following the below instruction.

[Install Vulkan on Raspberry Pi 4 - Q-engineering](https://qengineering.eu/install-vulkan-on-raspberry-pi.html)

## Run

```
DISPLAY=:0 HDMI_DEVICE="/dev/video0" MIC_MODE="ForceStereo" INGEST_SERVICE="Custom" RTMP_URL="rtmp://streaming.mzyy94.com/live/{stream_key}" STREAM_KEY="123456" cargo run --features "button-shim" -- --fullscreen
```
