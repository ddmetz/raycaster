run:
	cargo run

release:
	cargo build --release --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/raycaster.wasm docs

run-release:
	cargo run --release
	
