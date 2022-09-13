# Rustified IPv4 Heatmap

```bash
$ ipv4-heatmap 0.1.0
boB Rudis (@hrbrmstr)
Generate an IPv4 12th order Hilbert Heatmap from a file of IPv4 addresses.

USAGE:
    ipv4-heatmap [OPTIONS]

OPTIONS:
    -a, --annotations <ANNOTATIONS>    file containing JSON CIDR annotations
    -f, --filename <FILENAME>          input file of IPs [default: ips.txt]
    -h, --help                         Print help information
    -i, --invert                       invert the chosen color palette
    -l, --legend-file <LEGEND_FILE>    output an SVG colourbar legend to this file
    -o, --output <OUTPUT>              heatmap output file; extenstion determines format [default:
                                       map.png]
    -p, --palette <PALETTE>            color palette to use; one of (viridis magma inferno plasma
                                       cividis rocket mako turbo brbg puor rdbu rdgy rdylbu spectral
                                       bupu reds ylgnbu ylorbr ylorrd) [default: cividis]
    -r, --reverse                      reverse the heatmap base (i.e. white background, black text)
    -V, --version                      Print version information
```

![map](map.png)