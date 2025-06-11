BLUE=\033[0;34m
NC=\033[0m # No Color

.PHONY: all \
    build \
    build-release \
    check \
    check-cargo \
    check-clippy \
    check-fmt \
    check-pre-commit \
    check-updates \
    clean \
    doc \
    format \
    format-clippy \
    format-fmt \
    help \
    install \
    release \
    test

all: check build test build-release

install: ## Just install the tooling for the local dev environment
	@echo "\n${BLUE}pre-commit hook install and run...${NC}\n"
	pre-commit install

build: ## Build the code (dev)
	cargo build

build-release: ## Build the code (release)
	cargo build --release

check-pre-commit: ## Run pre-commit checks
	pre-commit run --all-files

check-clippy: ## Code linting with clippy
	cargo clippy

check-fmt: ## Code linting with rustfmt
	cargo fmt --check

check-cargo: ## Run cargo check
	cargo check

check-updates: ## Run cargo updates in dry mode to check for package updates
	cargo update --dry-run --verbose

check: check-pre-commit check-clippy check-fmt check-cargo check-updates ## Run all code checks

doc: ## Build and show code documentation
	cargo doc --no-deps --open

format-clippy: ## Apply clippy findings
	cargo clippy --fix --allow-staged --allow-dirty

format-fmt: ## Code formatting with rustfmt
	cargo fmt --all

format: format-clippy format-fmt ## Apply all code formatting

test: ## Run all unit tests
	cargo test

release: check format build test build-release ## Execute all the commands to release

clean: ## Force a clean environment: remove all temporary files and caches. Start from a new environment
	@echo "\n${BLUE}Cleaning up...${NC}\n"
	-rm -rf target

help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) \
		| sort \
		| awk 'BEGIN {FS = ":.*?## "}; \
		{printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
