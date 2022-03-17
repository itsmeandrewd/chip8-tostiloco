default:
	wasm-pack build --target web

test:
	cargo test

run:
	python3 -m http.server