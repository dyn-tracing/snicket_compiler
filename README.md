Repository for dynamic tracing queries.

## Dependencies
We primarily use Ubuntu 18.04 for our tests. For convenience, dependencies can be installed with the `setup.sh` command. It contains all the steps necessary to set up the compiler.

### Package Dependencies (Minikube)
- `rust` to build the dynamic tracing compiler
- `bazel` to build WASM filters for Envoy


## The Dyntracing Compiler (dtc)

### Building the Compiler
The compiler can be installed with `cargo install --path . --root . --no-track`.
After, the binary can be found in the `bin` folder.

### Compiling a Filter from a Query
```
`target/release/snicket -q [QUERY] -u [USER_DEFINED_FUNCTION]` 
```

# Examples
- Compile a counting query with the user defined function "height" and result will be written to `rust_filter/filter.rs`.
`target/release/snicket -q example_queries/height.cql -u example_udfs/height.rs -o rust_filter/filter.rs -c sim -r productpage-v1`

- Compile `service_name` query and result wiil be written to default location to `filter_envoy/filter.rs`
`target/debug/dtc -q example_queries/get_service_name.cql -o rust_filter/filter.rs -c sim -r productpage-v1`

