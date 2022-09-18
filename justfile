# This is a justfile (https://github.com/casey/just)

build:
	cargo build --release

doc:
	cargo doc --open --no-deps --target-dir docs/

docs:
	cargo doc --open --no-deps --target-dir docs/

codesign:
	cargo build --target=aarch64-apple-darwin --release && \
		cargo build --target=x86_64-apple-darwin --release && \
		lipo -create -output "${HOME}/bin/ipv4-heatmap" target/aarch64-apple-darwin/release/ipv4-heatmap target/x86_64-apple-darwin/release/ipv4-heatmap && \
		codesign --force --verify --verbose --sign "${APPLE_DEV_ID}" "${HOME}/bin/ipv4-heatmap"

# linux:
# 	TARGET_CC=x86_64-unknown-linux-gnu cargo build --target=x86_64-unknown-linux-gnu --release

deps:
	cargo deps | dot -Tsvg > assets/graph.svg
	extras/dot-embed-stylesheet.R
	cargo cyclonedx
	
test:
	cargo test --release

ge:
	cargo run --release -- --filename extras/ips.txt --output assets/ge.png --annotations extras/ge.json --invert --legend-file assets/legend.svg
	/usr/bin/open assets/ge.png

blues:
	cargo run --release -- --filename extras/ips.txt --palette blues --output assets/blues.png --annotations extras/iana.json --invert --legend-file assets/legend.svg
	/usr/bin/open assets/blues.png

invert:
	cargo run --release -- --filename extras/ips.txt --output assets/modern.png --annotations extras/iana-modern.json --invert --legend-file assets/legend.svg
	/usr/bin/open assets/modern.png

crop:
	cargo run --release -- --filename extras/ips.txt --output assets/pre-crop2.png --annotations extras/iana.json --invert --legend-file assets/legend.svg
	cargo run --release -- --filename extras/ips.txt --output assets/post-crop2.png --annotations extras/iana.json --invert --legend-file assets/legend.svg --crop 0.0.0.0/8,33.0.0.0/8
	/usr/bin/open assets/pre-crop2.png assets/post-crop2.png

oldschool:
	cargo run --release -- --filename extras/ips.txt --output assets/map.png --annotations extras/iana.json --invert --legend-file assets/legend.svg
	/usr/bin/open assets/map.png

example:
	cargo run --release -- --filename extras/ips.txt --output assets/map.png --annotations extras/iana-modern.json --legend-file extras/assets.svg
	/usr/bin/open assets/map.png

legend: example
	qlmanage -p assets/legend.svg