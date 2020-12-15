clean:
	rm -rf target/

build:
	#cargo build --release --target x86_64-unknown-linux-musl
	#cargo build --release --bin bootstrap
	docker run -v $$PWD:/build_dir -w /build_dir -t liuchong/rustup:musl cargo build --release --bin bootstrap

#docker:
	#docker run --rm --user "$$(id -u)":"$$(id -g)" -v $$PWD:/usr/src/myapp -w /usr/src/myapp rust cargo build --release
	#docker run --rm --user "$$(id -u)":"$$(id -g)" -v $$PWD:/usr/src/myapp -w /usr/src/myapp softprops/lambda-rust
	#docker run -v $$PWD:/build_dir -w /build_dir -t liuchong/rustup:musl cargo build --release --bin bootstrap

package:
	#zip -j target/rust.zip target/x86_64-unknown-linux-musl/release/bootstrap
	#mv target/release/lambda-poc target/bootstrap
	#zip -j target/rust.zip target/release/bootstrap
	zip -j target/rust.zip target/x86_64-unknown-linux-musl/release/bootstrap

sam-generate-events:
	sam local generate-event sqs receive-message | jq --arg body "Ben" '.Records[0].body = $$body' > event.json

run-sam:
	sam local invoke RustPoc --event event.json
