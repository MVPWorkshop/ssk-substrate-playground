# Rust toolchain
CARGO = cargo

# Targets
.PHONY: gen build run all

# Generate test_project
gen:
	$(CARGO) test build_all_pallets -- --ignored

# Navigate to the test_project directory and build it
build:
	cd generated_code/test_project && $(CARGO) build --release

# Navigate to test_project directory and run server
run:
	cd generated_code/test_project && ./target/release/solochain-template-node --dev

# Generate project with test, build it and run it on server
all: gen build run

# build test docker-compose
build_test:
	docker-compose -f docker-compose.test.yml build
# run integration tests
integration_tests:
	docker-compose -f docker-compose.test.yml up --abort-on-container-exit
	docker-compose -f docker-compose.test.yml down --remove-orphans