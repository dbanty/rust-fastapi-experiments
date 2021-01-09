This repo contains experimental Rust code written as research for [a blog series](https://dylananthony.com/posts/fastapi-rust-1-intro) about attempting to replace my usages of FastAPI with a Rust alternative.

## To Set Up (macOS)
1. Install AWS SAM (requires Docker): `brew tap aws/tap && brew install aws-sam-cli`
2. Add MUSL target: `rustup target add x86_64-unknown-linux-musl`
3. Install MUSL cross-compile tool: `brew install FiloSottile/musl-cross/musl-cross`
4. Soft link musl-gcc: `ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc`

## To Run the Code
1. Comment out any versions of the code you don't actually want to build in `template.yaml`
2. Run `sam build` and wait... it's going to take a _long time_ for the first build.
3. Run `sam local start-api` to start a local webserver which will serve the "lambdas". You must have Docker installed and running for this to work.
   
## A Tour
At the top level of this project is metadata which binds all the experiments together. Each experiment is its own crate, registered in the top level `Cargo.toml` workspace. If you want more information about a particular experiment, check out the README for that crate.

`template.yaml` is configuration data for AWS SAM which registers a lambda function for each experiment for use with API Gateway. Note that none of this code has necessarily been tested with a real Lambda/API Gateway, development is done using `sam local` to save some headache.

A `Makefile` is required in order to tell SAM how to build the crates. There is one entry per Lambda function called build-<nameOfFunction>.

## Resources
For more general information on deploying Rust code to AWS Lambda, check out [this blog post](https://dev.to/netguru/commentable-rs-building-a-serverless-comment-system-in-rust-5egb) and the [Netlify's fork of the AWS Rust Runtime](https://github.com/netlify/aws-lambda-rust-runtime).