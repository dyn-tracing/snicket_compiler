This compiles an example filter for envoy WASM.

# Build wasme
We use [wasme](https://github.com/solo-io/wasme) to build, push and deploy
our WASM filter.

However, it only supports deploying filters [`EnvoyFilter_SIDECAR_INBOUND`](https://pkg.go.dev/istio.io/api@v0.0.0-20191109011911-e51134872853/networking/v1alpha3?tab=doc#EnvoyFilter_PatchContext).
I have modified WASME code to deploy using `EnvoyFilter_ANY` and it's at [taegyunkim/wasme:patch-context](https://github.com/taegyunkim/wasme/tree/patch-context)

```
git clone -b patch-context git@github.com:taegyunkim/wasme.git
cd wasme
make wasme
cd _output
export PATH=$PWD:$PATH
```

# Build Filter
Install bazel https://docs.bazel.build/versions/master/install.html#installing-bazel
```
bazel-build :filter.wasm
```

Tag the filter and set the config.
```
wasme build precompiled ./bazel-bin/filter.wasm \
  --tag webassemblyhub.io/<your_username>/<filter_name>:<tag>  \
  --config runtime-config.json
```

To push and deploy, look at tutorials on WebAssembly Hub
https://docs.solo.io/web-assembly-hub/latest/tutorial_code/getting_started/
