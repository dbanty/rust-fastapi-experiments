build-RustApiActixFunction:
	cargo build --bin actix-web --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/actix-web $(ARTIFACTS_DIR)/bootstrap