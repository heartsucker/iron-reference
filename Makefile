.PHONY: help travis scss run
.DEFAULT_GOAL := help

help: ## Print this message and exit
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%16s\033[0m : %s\n", $$1, $$2}' $(MAKEFILE_LIST)

run: scss ## Run the server
	@cargo build && RUST_LOG=info cargo run

scss: ## Compile the SCSS into CSS
	@sass --scss --style expanded --sourcemap=none scss/styles.scss:static/css/styles.css || \
		echo -e 'scss not compiled :('

travis: ## Run the TravisCI tests
	@cargo test
