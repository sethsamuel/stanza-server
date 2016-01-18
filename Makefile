build:
	docker run -it --rm -v "$(PWD)":/src --volumes-from cargo-cache -w="/src" -p 3003:3003 fnichol/rust:1.5.0 cargo build --release
	docker build -t sethfsamuel/stanza-server .

run-debug:
	docker run -it --rm -v "$(PWD)":/src --volumes-from cargo-cache -w="/src" -p 3003:3003 fnichol/rust:1.5.0 cargo run --verbose

run:
	docker run -it --rm -p 3003:3003 sethfsamuel/stanza-server

all: build run