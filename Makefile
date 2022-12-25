.DEFAULT_GOAL := all 

all: compile try

compile:
	@cargo build --release
	@mkdir -p ./build/
	@cp ./target/release/stodo ./build/

clean:
	@cargo clean

overclean: clean
	@if [ -d "./build/" ]; then rm -rf ./build/; fi

try:
	@if [ -f "./build/stodo" ]; then ./build/stodo; fi
