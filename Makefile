# Makefile for common tasks in a Rust project
# Detect current branch
CURRENT_BRANCH := $(shell git rev-parse --abbrev-ref HEAD)
ZIP_NAME = OrderBook-rs.zip


# Default target
.PHONY: all
all: test fmt lint build

# Binding targets
.PHONY: bindings python nodejs test-bindings clean-bindings build-all test-all bench-all ci dev-python dev-nodejs install-python install-nodejs quick-test

# Build the project
.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

# Run tests
.PHONY: test
test:
	LOGLEVEL=WARN cargo test

# Format the code
.PHONY: fmt
fmt:
	cargo +stable fmt --all

# Check formatting
.PHONY: fmt-check
fmt-check:
	cargo +stable fmt --check

# Run Clippy for linting
.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint-fix
lint-fix: 
	cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged -- -D warnings

# Clean the project
.PHONY: clean
clean:
	cargo clean

# Pre-push checks
.PHONY: check
check: test fmt-check lint

# Run the project
.PHONY: run
run:
	cargo run

.PHONY: fix
fix:
	cargo fix --allow-staged --allow-dirty

.PHONY: pre-push
pre-push: fix fmt lint-fix test readme doc

.PHONY: doc
doc:
	cargo clippy -- -W missing-docs

.PHONY: doc-open
doc-open:
	cargo doc --open

.PHONY: publish
publish: readme
	find . -name ".DS_Store" -type f -delete | true
	cargo login ${CARGO_REGISTRY_TOKEN}
	cargo package
	cargo publish

.PHONY: coverage
coverage:
	export LOGLEVEL=WARN
	cargo install cargo-tarpaulin
	mkdir -p coverage
	cargo tarpaulin --exclude-files 'benches/**' --all-features --workspace --timeout 120 --out Xml

.PHONY: coverage-html
coverage-html:
	export LOGLEVEL=WARN
	cargo install cargo-tarpaulin
	mkdir -p coverage
	cargo tarpaulin --exclude-files 'benches/**' --verbose --all-features --workspace --timeout 120 --out Html

.PHONY: open-coverage
open-coverage:
	open tarpaulin-report.html

# Rule to show git log
git-log:
	@if [ "$(CURRENT_BRANCH)" = "HEAD" ]; then \
		echo "You are in a detached HEAD state. Please check out a branch."; \
		exit 1; \
	fi; \
	echo "Showing git log for branch $(CURRENT_BRANCH) against main:"; \
	git log main..$(CURRENT_BRANCH) --pretty=full

.PHONY: create-doc
create-doc:
	cargo doc --no-deps --document-private-items

.PHONY: readme
readme: check-cargo-readme create-doc
	cargo readme > README.md

.PHONY: check-cargo-readme
check-cargo-readme:
	@command -v cargo-readme > /dev/null || (echo "Installing cargo-readme..."; cargo install cargo-readme)

.PHONY: check-spanish
check-spanish:
	cd scripts && python3 spanish.py ../src && cd ..

.PHONY: zip
zip:
	@echo "Creating $(ZIP_NAME) without any 'target' directories, 'Cargo.lock', and hidden files..."
	@find . -type f \
		! -path "*/target/*" \
		! -path "./.*" \
		! -name "Cargo.lock" \
		! -name ".*" \
		| zip -@ $(ZIP_NAME)
	@echo "$(ZIP_NAME) created successfully."


.PHONY: check-cargo-criterion
check-cargo-criterion:
	@command -v cargo-criterion > /dev/null || (echo "Installing cargo-criterion..."; cargo install cargo-criterion)

.PHONY: bench
bench: check-cargo-criterion
	cargo criterion --output-format=quiet

.PHONY: bench-show
bench-show:
	open target/criterion/report/index.html

.PHONY: bench-save
bench-save: check-cargo-criterion
	cargo criterion --output-format quiet --history-id v0.3.2 --history-description "Version 0.3.2 baseline"

.PHONY: bench-compare
bench-compare: check-cargo-criterion
	cargo criterion --output-format verbose

.PHONY: bench-json
bench-json: check-cargo-criterion
	cargo criterion --message-format json

.PHONY: bench-clean
bench-clean:
	rm -rf target/criterion


.PHONY: workflow-coverage
workflow-coverage:
	DOCKER_HOST="$${DOCKER_HOST}" act push --job code_coverage_report \
       -P ubuntu-latest=catthehacker/ubuntu:latest \
       --privileged

.PHONY: workflow-build
workflow-build:
	DOCKER_HOST="$${DOCKER_HOST}" act push --job build \
       -P ubuntu-latest=catthehacker/ubuntu:latest

.PHONY: workflow-lint
workflow-lint:
	DOCKER_HOST="$${DOCKER_HOST}" act push --job lint

.PHONY: workflow-test
workflow-test:
	DOCKER_HOST="$${DOCKER_HOST}" act push --job run_tests

.PHONY: workflow
workflow: workflow-build workflow-lint workflow-test workflow-coverage

.PHONY: tree
tree: 
	tree -I 'target|.idea|.run|.DS_Store|Cargo.lock|*.md|*.toml|*.zip|*.html|*.xml|*.json|*.txt|*.sh|*.yml|*.yaml|*.gitignore|*.gitattributes|*.gitmodules|*.git|*.gitkeep|*.gitlab-ci.yml' -a -L 3

# Binding targets
bindings: python nodejs

python:
	@echo "Building Python bindings..."
	./scripts/build_bindings.sh python

nodejs:
	@echo "Building Node.js bindings..."
	./scripts/build_bindings.sh nodejs

test-bindings:
	@echo "Testing all bindings..."
	./scripts/build_bindings.sh test

clean-bindings:
	@echo "Cleaning binding artifacts..."
	./scripts/build_bindings.sh clean

# Combined targets
build-all: build bindings

test-all: test test-bindings

bench-all: bench
	@echo "Running binding benchmarks..."
	./scripts/build_bindings.sh bench

# Full build and test pipeline
ci: build-all test-all bench-all fmt-check lint

# Development helpers
dev-python:
	@echo "Building Python bindings for development..."
	maturin develop --features python

dev-nodejs:
	@echo "Building Node.js bindings for development..."
	npm run build

# Installation targets
install-python:
	@echo "Installing Python bindings..."
	pip install target/wheels/*.whl --force-reinstall

install-nodejs:
	@echo "Installing Node.js bindings..."
	npm install

# Quick development cycle
quick-test: build test-bindings

# Benchmark targets
bench-bindings:
	@echo "Running binding benchmarks..."
	./scripts/build_bindings.sh bench

bench-python:
	@echo "Running Python binding benchmarks..."
	python3 src/bindings/tests/benchmark_tests.py

bench-nodejs:
	@echo "Running Node.js binding benchmarks..."
	node --expose-gc src/bindings/tests/benchmark_tests.js

# Package targets
package-python:
	@echo "Packaging Python bindings..."
	maturin build --release --features python

package-nodejs:
	@echo "Packaging Node.js bindings..."
	npm pack

package-all: package-python package-nodejs