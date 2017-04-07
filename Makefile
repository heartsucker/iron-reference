.PHONY: help travis run
.DEFAULT_GOAL := help

help: ## Print this message and exit
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%16s\033[0m : %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run: ## Run the server
	@cargo build && RUST_LOG=info cargo run

travis: ## Run the TravisCI tests
	@cargo test
