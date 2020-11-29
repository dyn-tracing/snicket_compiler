#!/bin/bash
bazel build //:filter.wasm
#~/.wasme/bin/wasme build precompiled ./bazel-bin/filter.wasm --tag webassemblyhub.io/jb7399/message_counter:1  --config runtime-config.json
# wasme build cpp -t test-filter . --tag webassemblyhub.io/fruffy/test-filter:1 --config runtime-config.json
# wasme push webassemblyhub.io/fruffy/test-filter:1


