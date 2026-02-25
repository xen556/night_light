# night-light

A simple night light daemon for Hyprland (and other wlroots-based Wayland compositors) that reduces blue light.

> **Note:** Does **not** work on GNOME or KDE — they don't support the `zwlr_gamma_control_v1` protocol.

## How it works

Uses the `zwlr_gamma_control_v1` Wayland protocol to apply a warm gamma LUT to all connected monitors, reducing green and blue light while keeping red intact. The lower the blue light, the less melatonin is suppressed — making it easier to fall asleep.

## Requirements

- Hyprland (or any wlroots-based compositor: Sway, river, etc.)
- Rust + Cargo

## Installation

```bash
git clone https://github.com/yourusername/night-light
cd night-light
cargo build --release
sudo cp target/release/night-light /usr/local/bin/
```

## Usage

```bash
# Default (level 50)
night-light

# Custom level (1-100, higher = warmer/more orange)
night-light --level 80
```

Run in background:

```bash
night-light --level 75 &
```

To stop and restore normal colors:

```bash
killall night-light
```

## Autostart with Hyprland

Add to your `~/.config/hypr/hyprland.conf`:

```ini
exec-once = night-light --level 75
```

## Systemd user service

Create `~/.config/systemd/user/night-light.service`:

```ini
[Unit]
Description=Night Light

[Service]
ExecStart=/usr/local/bin/night-light --level 75
Restart=on-failure

[Install]
WantedBy=default.target
```

Enable it:

```bash
systemctl --user enable --now night-light
```

## Level guide

| Level | Effect |
|-------|--------|
| 1-20  | Very subtle, barely noticeable |
| 30-50 | Mild warm tint, similar to Windows Night Light |
| 60-80 | Strong orange, good for evening use |
| 90-100 | Very intense, ~2700K candlelight, maximum sleep benefit |

## Dependencies

- `wayland-client`
- `wayland-protocols-wlr`
- `memfd`
- `zerocopy`
- `clap`
