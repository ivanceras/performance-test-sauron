set -v

echo $@

cd client && \
wasm-pack build --release --target web -- $@ && \
cd ..
