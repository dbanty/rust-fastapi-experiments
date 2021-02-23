build:
	cargo build --bin rocket-api --release --target x86_64-unknown-linux-musl
	rm -rf bootstrap
	mkdir "bootstrap"
	cp ./target/x86_64-unknown-linux-musl/release/rocket-api bootstrap/bootstrap

deploy: build
	cdk deploy

local: build
	cdk synth --no-staging > template.yaml
	sam local start-api
