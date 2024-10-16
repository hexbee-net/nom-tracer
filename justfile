[private]
default:
  @just --list --no-aliases

# generate and open the documentation.
@doc:
  cargo doc --open

# build the library.
@build:
  cargo build --release

@bacon:
  bacon --all-features

# clean up everything
@clean:
  cargo clean

# reformat the code.
@fmt:
  cargo +nightly fmt --all

# runs all checks
check:
  just check-fmt
  just clippy
  just deny
  just test
  just udeps

# check if the code is formatted correctly.
@check-fmt:
  echo "Checking code format..."
  cargo +nightly fmt --all -- --check

# run clippy.
@clippy:
  echo "Running clippy..."
  cargo clippy --all-targets --all-features --tests -- -D warnings

# check dependencies licenses.
@deny:
  echo "Checking dependencies licenses..."
  cargo deny check

# check for unused dependencies.
@udeps:
  echo "Checking unused dependencies..."
  cargo +nightly udeps --all-targets

# run the unit tests.
@test:
  echo "Running tests..."
  cargo nextest run --all-features
