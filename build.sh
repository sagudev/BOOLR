#!/bin/sh
set -ex
wasm-pack build --target web
#python3 -m http.server
# post-bindgen
#sed -i 's/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/OK/g' test
cp ./pkg/BOOLR.js ./pkg/BOOLR.js.bk
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! == '\''function'\'' ? ! == '\''function'\'' ? function(){ !g}' ./pkg/BOOLR.js
sed -i '/typeof .* == '\''function'\'' ? .* : notDefined('\''.*'\'');/{s! : notDefined('\''!(); } : notDefined('\''!g}' ./pkg/BOOLR.js
diff ./pkg/BOOLR.js ./pkg/BOOLR.js.bk