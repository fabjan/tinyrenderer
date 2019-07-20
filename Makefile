test:
	set -e;\
	RENDER="render.$$(date +%y-%m-%d_%H:%M:%S).pbm";\
	cargo run --release > "$$RENDER"

build:
	cargo build --release

compact:
	cksum render.*.pbm | while read sum size name; do mv "$$name" "$$(echo 'obase=16;'$$sum | bc).pbm"; done

.PHONY: build test compact
