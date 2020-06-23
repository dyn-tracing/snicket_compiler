This compiles an example filter for envoy WASM.

# Install bazel
https://docs.bazel.build/versions/master/install.html#installing-bazel

# build filter
build with
```
bazel build :filter.wasm
```

Filter will be in:
```
./bazel-bin/filter.wasm
```

Tag the filter and set the config.
```
wasme build precompiled ./bazel-bin/filter.wasm \
  --tag webassemblyhub.io/<your_username>/<filter_name>:<tag>  \
  --config runtime-config.json
```

To push and deploy, look at tutorials on WebAssembly Hub
https://docs.solo.io/web-assembly-hub/latest/tutorial_code/getting_started/
