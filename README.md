Repository for dynamic tracing queries.

## Dependencies
This repository relies on Minikube and GCP to emulate microservice scenarios. We primarily use Ubuntu 18.04 for our tests. For convenience, dependencies can be installed with the `setup.sh` command. It contains all the steps necessary to set up the compiler.

### Package Dependencies (Minikube)
- `rust` to build the dynamic tracing compiler
- `bazel` to build WASM filters for Envoy


## The Dyntracing Compiler

### Building the Compiler
The compiler can be build with `cargo build`.

### Compiling a Filter from a Query
After the compiler has been built, you can compile queries with the
`target/debug/dyntracing -q [QUERY] -u [USER_DEFINED_FUNCTION]` command. For example,
`target/debug/dyntracing -q example_queries/count.cql -u example_udfs/count.cc` compiles a counting query with the user defined function "count". The result will be stored in `cpp_filter/filter.cc`.



