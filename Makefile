all: release

debug:
	cargo build

release:
	cargo build --release

install:
	cp target/release/syssnapshot /usr/local/bin
