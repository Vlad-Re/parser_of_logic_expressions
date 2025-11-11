test:
	@echo "Running tests..."
	@cargo test

run:
	@echo "Running program..."
	@cargo run

fmt:
	@echo "Formatting code..."
	@cargo fmt

clippy:
	@echo "Running clippy..."
	@cargo clippy

build:
	@echo "Building debug..."
	@cargo build

release:
	@echo "Building releas..."
	@cargo build --release

doc:
	@echo "Opening documentation..."
	@cargo doc --open

clean:
	@echo "Cleaning..."
	@cargo clean
