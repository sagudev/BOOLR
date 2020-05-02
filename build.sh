#!/bin/sh
set -e
# Run format
cargo fmt
# Build
wasm-pack build --target web
# Server
#python3 -m http.server
# post-bindgen
# allow dialog.hide as js_namespace
# TODO: no post-bindgen
#sed -i 's/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/OK/g' test
# Back up
cp ./pkg/BOOLR.js ./pkg/BOOLR.js.bk
# Do post-bindgen
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! == '\''function'\'' ? ! == '\''function'\'' ? function(){ !g}' ./pkg/BOOLR.js
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! : notDefined('\''!(); } : notDefined('\''!g}' ./pkg/BOOLR.js
# What's changed  in postgen
diff ./pkg/BOOLR.js ./pkg/BOOLR.js.bk