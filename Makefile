build:
	cargo build --bin rocket-api --release --target x86_64-unknown-linux-musl

build-RocketFunction: build
	cp ./target/x86_64-unknown-linux-musl/release/rocket-api $(ARTIFACTS_DIR)/bootstrap

deploy-cdk: build
	mkdir "bootstrap"
	cp ./target/x86_64-unknown-linux-musl/release/rocket-api bootstrap/bootstrap
	# zip -r bootstrap.zip bootstrap
	cd cdk && cdk deploy --profile personal
	rm -rf bootstrap
