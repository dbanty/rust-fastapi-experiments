build-RocketFunction:
	cargo build --bin rocket-api --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/rocket-api $(ARTIFACTS_DIR)/bootstrap