PWD = pwd
cargo build --release
pkexec cp $PWD/target/release/ocrint /usr/bin/
