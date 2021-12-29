clean:
	cargo clean
	rm -rf ui/*

build:
	cargo build
	cd ui-src && npm run build

run: build
	cargo run