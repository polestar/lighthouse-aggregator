all: check test build

test:
	cargo test -q

build: test check
	cargo build --release

check:
	cargo check

install:
	cp target/release/lighthouse-groupie /usr/local/bin

