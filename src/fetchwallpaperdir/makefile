
export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_STATIC=true
export OPENSSL_DIR=/musl


build:
	cargo build --target x86_64-unknown-linux-musl --release
	rm ../fetchwallpaper
	mv target/x86_64-unknown-linux-musl/release/project ../fetchwallpaper

move:
	cp target/x86_64-unknown-linux-musl/release/project ../fetchwallpaper
