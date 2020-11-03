Repository for dynamic tracing queries.

Use cypher patterns as a basis for specifying desired trace attributes: https://neo4j.com/docs/2.0/cypher-refcard/

# Install

- Rust nightly: `rustup toolchain install nightly`
- [Bazel](https://docs.bazel.build/versions/master/install.html)
- [Google Cloud SDK](https://cloud.google.com/sdk/install)
- [Docker](https://www.docker.com/products/docker-desktop)
- [Wasme CLI](https://docs.solo.io/web-assembly-hub/latest/tutorial_code/getting_started/#install-the-wasme-cli)
(use the patched version below).

# git clone this repository

# Demo

1. Setup GKE cluster

First create a GCP project if you don't have one.

```
gcloud config set project <PROJECT_ID>

gcloud services enable container.googleapis.com

gcloud container clusters create demo --enable-autoupgrade \
    --enable-autoscaling --min-nodes=3 --max-nodes=10 \
		--num-nodes=5 --zone=us-central1-a

# Retrieve your credentials for `kubectl`
gcloud container clusters get-credentials $CLUSTER_NAME \
    --zone $ZONE
```

2. Install Istio

```
curl -L https://istio.io/downloadIstio | ISTIO_VERSION=1.5.4 sh -

cd istio-1.5.4

bin/istioctl manifest apply --set profile=demo
```

3. Enable istio injection for the default namespace

```
kubectl label namespace default istio-injection=enabled
```

4. Deploy patched bookinfo application.

```
git clone -b bookinfo-headers https://github.com/taegyunkim/istio.git

cd istio

kubectl apply -f samples/bookinfo/platform/kube/bookinfo.yaml
```

Confirm the bookinfo application is running.

```
kubectl exec -it $(kubectl get pod -l app=ratings -o jsonpath='{.items[0].metadata.name}') -c ratings -- curl productpage:9080/productpage | grep -o "<title>.*</title>"
```

Define the ingress IP and port

```
kubectl apply -f samples/bookinfo/networking/bookinfo-gateway.yaml
```

Set the ingress IP and ports

```
export INGRESS_HOST=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
export INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].port}')
export SECURE_INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="https")].port}')
```

Set the `GATEWAY_URL`

```
export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT
```

5. Confirm the app is accessible from outside the cluster

```
curl -s http://${GATEWAY_URL}/productpage | grep -o "<title>.*</title>"
```

6. Deploy storage-upstream (the wasm folder here refers to the wasm folder within the current repository)

```
kubectl apply -f wasm/storage_upstream.yaml
```

7. Configure storage upstream cluster in productpage service

```
kubectl apply -f wasm/productpage-cluster.yaml
```

8. To enable Jaeger trace collections, run
```
kubectl create namespace observability
kubectl create -n observability -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/crds/jaegertracing.io_jaegers_crd.yaml
kubectl create -n observability -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/service_account.yaml
kubectl create -n observability -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/role.yaml
kubectl create -n observability -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/role_binding.yaml
kubectl create -n observability -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/operator.yaml
kubectl create -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/cluster_role.yaml
kubectl create -f https://raw.githubusercontent.com/jaegertracing/jaeger-operator/master/deploy/cluster_role_binding.yaml
```

and then create a file called jaeger.yaml with the following contents:
```
apiVersion: jaegertracing.io/v1
kind: Jaeger
metadata:
  name: jaeger
spec:
  query:
    serviceType: NodePort
  strategy: allInOne # <1>
  allInOne:
    image: jaegertracing/all-in-one:latest # <2>
    options: # <3>
      query.max-clock-skew-adjustment: 0 # <4>
```

and finally run
```
kubectl apply -n observability jaeger.yaml
kubectl get ingress -n observability
```
The output from the last command will contain an IP address at which you can access the Jaeger UI.

8. Generate `wasm/filter.cc` file by running `cargo run`

For example,
`cargo run -- -q ./example_queries/return/query.cql`

9. Build WASME
   We use [wasme](https://github.com/solo-io/wasme) to build, push and deploy
   our WASM filter.

However, it only supports deploying filters [`EnvoyFilter_SIDECAR_INBOUND`](https://pkg.go.dev/istio.io/api@v0.0.0-20191109011911-e51134872853/networking/v1alpha3?tab=doc#EnvoyFilter_PatchContext).
I have modified WASME code to deploy using `EnvoyFilter_ANY` and it's at [taegyunkim/wasme:patch-context](https://github.com/taegyunkim/wasme/tree/patch-context)

```
git clone -b patch-context https://github.com/taegyunkim/wasme.git
cd wasme
make wasme
cd _output
export PATH=$PWD:$PATH
```

10. Build and deploy Filter
    
    If needed, install bazel https://docs.bazel.build/versions/master/install.html#installing-bazel

```
cd wasm
bazel build :filter.wasm
```

Tag the filter and set the config.

```
wasme build precompiled ./bazel-bin/filter.wasm \
  --tag webassemblyhub.io/<your_username>/<filter_name>:<tag>  \
  --config runtime-config.json
```

Push the filter to WebAssembly Hub

```
wasme login

wasme push webassemblyhub.io/<your_username>/<filter_name>:<tag>
```

Deploy the filter

```
wasme deploy istio webassemblyhub.io/<your_username>/<filter_name>:<tag> --id=<set an appropriate id> --namespace=default
```

11. Make few requests (using step 5 above or accessing the productpage via a browser) and check contents in storage-upstream using the command below.


```
kubectl exec -it $(kubectl get pod -l app=ratings -o jsonpath='{.items[0].metadata.name}') -c ratings -- curl -v storage-upstream:8080/list
```
