#!/bin/bash
source "/home/andy/.bash_aliases"
shopt -s expand_aliases

#build (uses default LLVM optimization whic is speed)
wasm-pack build --target web

#optimize with wasm-opt (Binaryen toolkit) which goes further than LLVM's WebAssembly backend
# -O2 might be the better option? see wasm-opt --help
wasm-opt pkg/albo_server_bg.wasm -O4 -o pkg/optimized.wasm

#start server
deno run --allow-net --allow-read main.ts