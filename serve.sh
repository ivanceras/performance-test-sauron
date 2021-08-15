set -e

./build.sh

cargo run --release --bin server $a
