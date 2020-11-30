#!/bin/bash
FILE_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

cd ${FILE_DIR}/wasm/tools/wasme/cli
make _output/wasme
if [ $? -eq 0 ]; then
    mv _output/wasme ../../../../bin/wasme_patched
fi

cd -
