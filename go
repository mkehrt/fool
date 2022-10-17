#!/bin/bash

find src -name *.rs | xargs rustfmt &&
wasm-pack build --target web &&
python3 -m http.server

