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

infra8:
	cargo run --release -- --palette magma --filename extras/ips.txt --output assets/infra8s.png --annotations extras/infra-and-slash-8.json --invert
	/usr/bin/open assets/infra8s.png

mask:
	cargo run --release -- --palette magma --invert --filename extras/ips.txt --annotations extras/mask-labels.json --output assets/mask.png --mask 109.0.0.0/8,141.0.0.0/8,145.0.0.0/8,151.0.0.0/8,176.0.0.0/8,178.0.0.0/8,185.0.0.0/8,188.0.0.0/8,193.0.0.0/8,194.0.0.0/8,195.0.0.0/8,2.0.0.0/8,212.0.0.0/8,213.0.0.0/8,217.0.0.0/8,31.0.0.0/8,37.0.0.0/8,46.0.0.0/8,5.0.0.0/8,51.0.0.0/8,57.0.0.0/8,62.0.0.0/8,77.0.0.0/8,78.0.0.0/8,79.0.0.0/8,80.0.0.0/8,81.0.0.0/8,82.0.0.0/8,83.0.0.0/8,84.0.0.0/8,85.0.0.0/8,86.0.0.0/8,87.0.0.0/8,88.0.0.0/8,89.0.0.0/8,90.0.0.0/8,91.0.0.0/8,92.0.0.0/8,93.0.0.0/8,94.0.0.0/8,95.0.0.0/8
	open assets/mask.png

example:
	cargo run --release -- --filename extras/ips.txt --output assets/map.png --annotations extras/iana-modern.json --legend-file extras/assets.svg
	/usr/bin/open assets/map.png

legend: example
	qlmanage -p assets/legend.svg