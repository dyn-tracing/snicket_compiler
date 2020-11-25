#!/bin/bash
cd wasm/tools/wasme/cli
make _output/wasme
cp _output/wasme ../../../../bin/wasme_patched
cd -
