# This is a justfile (https://github.com/casey/just)

build:
	cargo build --release

test:
	cargo test --release

example:
	cargo run --release -- --annotations extras/iana.json --invert --legend-file extras/legend.svg

inspect: example
	/usr/bin/open map.png
	/usr/bin/qlmanage -p extras/legend.svg