# This is a justfile (https://github.com/casey/just)

build:
	cargo build --release

doc:
	cargo doc --no-deps --target-dir docs/

codesign:
	cargo build --target=aarch64-apple-darwin --release && \
		cargo build --target=x86_64-apple-darwin --release && \
		lipo -create -output "${HOME}/bin/ipv4-heatmap" target/aarch64-apple-darwin/release/ipv4-heatmap target/x86_64-apple-darwin/release/ipv4-heatmap && \
		codesign --force --verify --verbose --sign "${APPLE_DEV_ID}" "${HOME}/bin/ipv4-heatmap"

# linux:
# 	TARGET_CC=x86_64-unknown-linux-gnu cargo build --target=x86_64-unknown-linux-gnu --release

deps:
	cargo deps | dot -Tpng > graph.png
	cargo cyclonedx
	
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

legend: example
	/usr/bin/open extras/legend.svg