build:
	wasm-pack build --target web

serve:
	python3 -m http.server

run: build serve

all: build serve
