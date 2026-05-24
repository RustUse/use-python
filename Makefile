.PHONY: help fmt check lint test build doc examples publish-dry-run-focused publish-dry-run-facade release-readiness verify

FOCUSED_CRATES := use-python-keyword use-python-version use-python-value use-pyproject use-pip use-uv use-venv use-python-identifier use-python-module use-pytest
FACADE_CRATE := use-python

help:
	@printf "%s\n" \
		"help                    Show available repository tasks" \
		"fmt                     Check formatting with rustfmt" \
		"check                   Run cargo check for the workspace" \
		"lint                    Run clippy with warnings denied" \
		"test                    Run workspace tests with all features" \
		"build                   Build the workspace with all features" \
		"doc                     Build workspace docs without dependencies" \
		"examples                Check all examples" \
		"publish-dry-run-focused Dry-run publish focused crates" \
		"publish-dry-run-facade  Dry-run publish $(FACADE_CRATE) after crates.io propagation" \
		"release-readiness       Run the pre-release focused-crate validation path" \
		"verify                  Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

publish-dry-run-focused:
	@if [ -z "$(strip $(FOCUSED_CRATES))" ]; then \
		printf "%s\n" "No focused crates configured"; \
	else \
		for crate in $(FOCUSED_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples publish-dry-run-focused

verify: fmt lint test build
