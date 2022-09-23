A simple tool suitable for adjusting external monitor's brightness.

Much faster than [ddcutil](https://github.com/rockowitz/ddcutil), still faster than [ddcset](https://github.com/arcnmx/ddcset-rs) which enumerates all monitors. And it matches monitor by the output name you see from xrandr / wayland-info output.

To install, install rust and run `cargo build --release` and find the binary in `target/release/`.

Usage:

```
monitor-control 0.2.0
lilydjwg <lilydjwg@gmail.com>
The fastest way to get / set DDC values for a monitor

USAGE:
    monitor-control <OUTPUT_NAME> <FEATURE_CODE> [FEATURE_VALUE]

ARGS:
    <OUTPUT_NAME>      output name such as DP-1
    <FEATURE_CODE>     feature code in decimal or 0xFF or FFh format
    <FEATURE_VALUE>    value to be set; when not present show current value

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

E.g.

```sh
# get current and max brightness value
monitor-control DP-2 16
# set brightness to 50
monitor-control DP-2 16 50
```

<small>Scripts to determine which monitor to adjust and show indicators like [wob](https://github.com/francma/wob) are left to other projects.</small>
