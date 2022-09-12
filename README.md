# Rustified IPv4 Heatmap

```bash
Generate an IPv4 12th order Hilbert Heatmap from a file of IPv4 addresses.

USAGE:
    ipv4-heatmap [OPTIONS]

OPTIONS:
    -f, --filename <FILENAME>    filename of ips [default: ips.txt]
    -h, --help                   Print help information
    -i, --invert                 invert color palette
    -o, --output <OUTPUT>        map output filename [default: map.png]
    -p, --palette <PALETTE>      color palette; one of (viridis brbg puor rdbu rdgy rdylbu spectral
                                 bupu reds ylgnbu ylorbr ylorrd) [default: viridis]
    -r, --reverse                reverse; white background, black text
    -V, --version                Print version information
```

![map](map.png)