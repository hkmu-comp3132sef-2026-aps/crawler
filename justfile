set shell := ["bash", "-cu"]
set windows-shell := ["powershell"]

# Default action
_:
    just lint
    just fmt

# Lint code
lint:
    ls-lint
    typos
    cargo check
    cargo clippy
    cargo test -- --nocapture

# Format code
fmt:
    cargo fmt

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
