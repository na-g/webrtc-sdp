.PHONEY: wasm parcel
# For installation instructions for the various tools see https://github.com/alexcrichton/wasm-bindgen/blob/master/README.md
wasm:
	cargo build --lib --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/rsdparsa.wasm --out-dir .
	wasm2es6js rsdparsa_wasm.wasm -o rsdparsa_wasm.js --base64
parcel: wasm
	parcel index.html
