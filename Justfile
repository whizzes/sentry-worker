set dotenv-load
set positional-arguments

# List Tasks
default:
    just --list

dev:
    bun run dev

# Perform formatting and linting
fmt:
    cargo clippy --fix --workspace --allow-dirty --allow-staged && cargo fmt
