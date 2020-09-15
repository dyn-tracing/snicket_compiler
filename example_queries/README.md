# Example Queries

Each `.cql` query file has a corresponding `_filter.cc` file and the filter
file contains what command was used to generate it.

To compile a `*_filter.cc` file

```
cp example_filter.cc ../wasm/filter.cc
cd ../wasm
bazel build filter.wasm
```