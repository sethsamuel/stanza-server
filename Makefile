build:
	docker run -it --rm -v "$(PWD)":/src --volumes-from cargo-cache -w="/src" -p 3003:3003 fnichol/rust:1.5.0 cargo build --release
	docker build -t stanza-server .

run:
	docker run -it --rm -p 3003:3003 stanza-server