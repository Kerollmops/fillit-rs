# https://doc.rust-lang.org/rustc/profile-guided-optimization.html

# STEP 0: Make sure there is no left-over profiling data from previous runs
rm -rf /tmp/pgo-data

# STEP 1: Build the instrumented binaries
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    cargo build --release

# STEP 2: Run the instrumented binaries with some typical data
cat tests/valid_hard_forum_18.fillit | ./target/release/fillit
cat tests/valid_hard_forum_20.fillit | ./target/release/fillit
# cat tests/valid_hard_forum_23.fillit | ./target/release/fillit

# STEP 3: Merge the `.profraw` files into a `.profdata` file
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# STEP 4: Use the `.profdata` file for guiding optimizations
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
    cargo build --release
