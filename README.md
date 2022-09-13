# Rustified IPv4 Heatmap

```bash
$ ipv4-heatmap --help
ipv4-heatmap 0.1.0
boB Rudis (@hrbrmstr)
Generate an IPv4 12th order Hilbert Heatmap from a file of IPv4 addresses.

USAGE:
    ipv4-heatmap [OPTIONS]

OPTIONS:
    -a, --annotations <ANNOTATIONS>    annotations file
    -f, --filename <FILENAME>          file of IPs [default: ips.txt]
    -h, --help                         Print help information
    -i, --invert                       invert color palette
    -o, --output <OUTPUT>              map output file [default: map.png]
    -p, --palette <PALETTE>            color palette; one of (viridis magma inferno plasma cividis
                                       rocket mako turbo brbg puor rdbu rdgy rdylbu spectral bupu
                                       reds ylgnbu ylorbr ylorrd) [default: magma]
    -r, --reverse                      reverse (white background, black text)
    -V, --version                      Print version information```

![map](map.png)