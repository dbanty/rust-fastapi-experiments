An experiment for running actix-web + Paperclip on AWS Lambda.

## To Set Up (macOS)
1. Install AWS SAM (requires Docker): `brew tap aws/tap && brew install aws-sam-cli`
2. Add MUSL target: `rustup target add x86_64-unknown-linux-musl`
3. Install MUSL cross-compile tool: `brew install FiloSottile/musl-cross/musl-cross`
4. Soft link musl-gcc: `ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc`