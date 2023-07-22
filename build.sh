set -ev

echo $@

cd client && \
wasm-pack build --release --target web -- $@ && \
cd ..
