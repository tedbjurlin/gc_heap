default:
    @just --list

docs:
  cargo doc --open

# Run nix develpment shell
develop *ARGS:
  nix develop {{ARGS}}

# Autoformat the project tree
fmt:
  cargo fmt

# Run rust project and build for docker
run:
    nix run
