#!/bin/bash
cd wasm/tools/wasme/cli
make _output/wasme
if [ $? -eq 0 ]; then
    mv _output/wasme ../../../../bin/wasme_patched
fi

cd -
