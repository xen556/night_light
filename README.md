# night-light

A minimal night light daemon for wlroots-based Wayland compositors.
Applies a warm gamma curve to all connected monitors using the
`zwlr_gamma_control_v1` protocol.

> Note: Does NOT work on GNOME or KDE (they do not support this protocol).

---

## Requirements

- wlroots-based compositor (Hyprland, Sway, river, etc.)
- Linux
- Rust + Cargo

---

## Installation

git clone https://github.com/yourusername/night-light
cd night-light
cargo build --release
sudo cp target/release/night-light /usr/local/bin/

---

## Usage

Set level (0–100):

night-light --level 70

Default level: 50

The level is stored in:

/tmp/night-light

Run daemon:

night-light --daemon

Daemon behavior:
- Connects to Wayland
- Detects all wl_output monitors
- Creates gamma control per output
- Reads /tmp/night-light every second
- Regenerates and reapplies gamma tables

---

## Level Guide

0        Neutral (no warmth)
20       Slightly warmer
40–60    Comfortable evening warmth
70–80    Strong orange tint
90–100   Very warm, minimal blue light

---

## Autostart (Hyprland)

Add to:
~/.config/hypr/hyprland.conf

exec-once = night-light --daemon

---

## Systemd User Service

Create:
~/.config/systemd/user/night-light.service

[Unit]
Description=Night Light

[Service]
ExecStart=/usr/local/bin/night-light --daemon
Restart=on-failure

[Install]
WantedBy=default.target

Enable:

systemctl --user enable --now night-light

---

## Dependencies

- wayland-client
- wayland-protocols-wlr
- memfd
- zerocopy
- clap

---
