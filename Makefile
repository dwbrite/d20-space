clean:
	cargo clean
	rm -rf ui/*

install:
	cd ui-src && npm install

build-backend:
	cargo build

build-frontend:
	cd ui-src && npm run build

build: build-backend build-frontend

run: build
	cargo run