debug:
	cargo build

release:
	cargo build --release

install: release
	cp target/release/syssnapshot /usr/local/bin
