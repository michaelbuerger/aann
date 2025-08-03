all:
	@echo "make run   |  Runs executable target (example_network_1.rs)"
	@echo "make run2  |  Runs executable target (example_network_2.rs)"
	@echo "make fmt   |  Formats all source code"
	@echo "make test  |  Runs all tests"

run:
	cargo run --bin example_network_1

run2:
	cargo run --bin example_network_2

fmt:
	cargo fmt

test:
	cargo test