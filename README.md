Repository for dynamic tracing queries.

## Dependencies
This repository relies on Minikube and GCP to emulate microservice scenarios. We primarily use Ubuntu 18.04 for our tests. For convenience, dependencies can be installed with the `./tools/setup.sh` command. It contains all the steps necessary to set up the compiler and testing environment.

### Package Dependencies (Minikube)
- `rust` to build the dynamic tracing compiler
- `kubernetes` to administrate and run web applications in a cluster
- `minikube` to run a local Kubernetes cluster
- `docker` as the container driver back end of Minikube
- `bazel` to build Wasm filters for Envoy
- `istio` to manage Envoy and its filters
- `python3.6` to run the scripts

### Additional Package Dependencies (GCP)
TBD

### Python Dependencies
- `prometheus-api-client` to query Prometheus

## The Dyntracing Compiler

### Building the Compiler
The compiler can be build with `cargo build`.

### Compiling a Filter from a Query
After the compiler has been built, you can compile queries with the
`target/debug/dyntracing -q [QUERY] -u [USER_DEFINED_FUNCTION]` command. For example,
`target/debug/dyntracing -q example_queries/count.cql -u example_udfs/count.cc` compiles a counting query with the user defined function "count". The result will be stored in `cpp_filter/filter.cc`.

## The Testing Environment

### Environment Setup
Once everything is installed, the Kubernetes cluster can be started with
`./kube_env.py --setup` if you are running on minikube or `./kube_env.py --setup -p GCP` if you are running on google cloud. If you are running on google cloud, add `-p GCP` to all subsequent commands as well.  Starting the cluster will take a few minutes.

### Building a filter
We provide a pre-built filter. It can be deployed with the `./kube_env.py --deploy-filter ` command. If the filter was generated from the compiler, it
can be rebuilt with the `./kube_env.py --build-filter ` command

### Testing the filter
Once the filter has been successfully installed, it is possible to run experiments with  `./run_experiments.py --num-experiments 1`. You can also issue single
HTTP requests with `./env/send_request.py`.

### Teardown
Remove the filter

`./kube_env.py --undeploy-filter`

Remove the deployment

`./kube_env.py --remove-bookinfo`

Tear down the cluster

`./kube_env.py --clean`


