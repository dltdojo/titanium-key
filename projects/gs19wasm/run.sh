#!/bin/bash
#set -x

cd "$(dirname "$0")"

VER=190412-1

build(){
    wasm-pack build --target web
    rm -rf web/dist
    mkdir -p web/dist
    cp pkg/gs19wasm_bg.wasm web/dist/gs19wasm-${VER}.wasm
    cp pkg/gs19wasm.js web/dist/gs19wasm-${VER}.js
    cp index.html web/dist/index.html
    sed -i "s|./pkg/gs19wasm.js|./gs19wasm-${VER}.js|" web/dist/index.html
    sed -i "s|./pkg/gs19wasm_bg.wasm|gs19wasm-${VER}.wasm|" web/dist/index.html
    tree -lah web
}

test-web(){
    pushd web/dist
    lite-server
    popd
}

dev-web(){
    lite-server
}

case "$1" in 
    build)   build ;;
    test-web)   test-web ;;
    dev-web)   dev-web ;;
    *) echo "usage: $0 build|test-web|dev-web" >&2
       exit 1
       ;;
esac
