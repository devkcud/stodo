compile:
	@cargo build --release
	@cp ./target/release/stodo ./build/
	./build/stodo --version