#!/bin/sh
set -e
# Run format
cargo fmt
# Build
case "$1" in
    r*)
        wasm-pack build --target web --release
    ;;
    *)
        wasm-pack build --target web --dev
    ;;
esac
# Server
#python3 -m http.server
# post-bindgen
# allow dialog.hide as js_namespace
# TODO: no post-bindgen
#sed -i 's/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/OK/g' test
# Back up
cp ./pkg/BOOLR.js ./pkg/BOOLR.js.bk
# Do post-bindgen
# tel namespace ni glih za dialog.show ampak če nardim sed k bo mal popravu boolr.js bo vse ok.
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! == '\''function'\'' ? ! == '\''function'\'' ? function(){ !g}' ./pkg/BOOLR.js
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! : notDefined('\''!(); } : notDefined('\''!g}' ./pkg/BOOLR.js
# What's changed  in postgen
diff ./pkg/BOOLR.js ./pkg/BOOLR.js.bk