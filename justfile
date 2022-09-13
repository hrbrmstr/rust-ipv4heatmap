# This is a justfile (https://github.com/casey/just)

build:
	cargo build --release

test:
	cargo test --release

ge:
	cargo run --release -- --annotations extras/ge.json --invert --legend-file extras/legend.svg
	/usr/bin/open map.png

invert:
	cargo run --release -- --annotations extras/iana.json --invert --legend-file extras/legend.svg
	/usr/bin/open map.png

example:
	cargo run --release -- --annotations extras/iana.json --legend-file extras/legend.svg
	/usr/bin/open map.png

inspect: example
	/usr/bin/qlmanage -p extras/legend.svg