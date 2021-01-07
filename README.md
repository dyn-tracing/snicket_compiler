Repository for dynamic tracing queries.

## Install
This repository relies on Minikube and GCP to emulate microservice scenarios. We primarily use Ubuntu 18.04 for our tests. For convenience, dependencies can be installed with the `./tools/setup.sh` command. It contains all the steps necessary to set up the testing environment.

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

## Setup
Once everything is installed, the Kubernetes cluster can be started with
`./kube_env.py --setup` if you are running on minikube or `./kube_env.py --setup -p GCP` if you are running on google cloud. If you are running on google cloud, add `-p GCP` to all subsequent commands as well.  Starting the cluster will take a few minutes.

## Building a filter
We provide a pre-built filter. It can be deployed with the `./kube_env.py --deploy-filter ` command.


## Demo
Once the filter has been successfully installed, it is possible to run experiments with  `./run_experiments.py --num-experiments 1`.

## Teardown
Remove the filter

`./kube_env.py --undeploy-filter`

Remove the bookinfo application

`./kube_env.py --remove-bookinfo`

Tear down the cluster

`./kube_env.py --clean`


