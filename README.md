Night-light is lightweight and native tool for wayland.

## Installation
```
1. git clone https://github.com/xen556/night_light
2. cd night_light
3. cargo build --release

or just download program from release ;D
```
----

## Usage:
```
night-light [OPTIONS]

Options:
      --level <LEVEL>  1-100 [default: 50]
      --daemon         
  -h, --help           Print help

1. Open terminal and run night-light --daemon
2. In second terminal use "night-light --level 60" to change temperature

Example
1. night-light --daemon
2. night-light --level 44
```

----

# Dependencies:
```
clap = { version = "4.5.60", features = ["derive"] }
memfd = "0.6.5"
wayland-client = "0.31.11"
wayland-protocols-wlr = { version = "0.3", features = ["client"] }
zerocopy = "0.8.39"
```
----


