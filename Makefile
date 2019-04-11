all:
	cargo build --release

install:
	install -m 644 include/libmonsterengine.h /usr/include
	install target/release/libmonsterengine.so /usr/lib

clean:
	cargo clean
