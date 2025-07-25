build_app: build_lib
	cd app && npx tsc
watch_app: build_lib
	cd app && npx tsc --watch

build_lib:
	cd lib && wasm-pack build --target web
watch_lib:
	cd lib && cargo watch -s "wasm-pack build --target web"