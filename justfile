set shell := ["bash", "-cu"]
set windows-shell := ["pwsh", "-Command"]

# Default action
_:
    just --list -u

# Format code
fmt:
    cargo fmt

# Lint code with ls-lint
ls-lint:
    ls-lint -config ./.ls-lint.yaml

# Lint code with ls-lint
lslint:
    just ls-lint

# Lint code with typos-cli
typos:
    typos

# Lint code
lint:
    just lslint
    just typos
    cargo check
    cargo clippy
    cargo test -- --nocapture

# Check code
check:
    just fmt
    just lint

# Run in development
dev:
    RUST_ENV=development cargo run

# Run in test
tst:
    RUST_ENV=test cargo run

# Run in production
prd:
    RUST_ENV=production cargo run

# Clean up
clean:
    cargo clean
