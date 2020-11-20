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

1. Change your directory to deps
```
cd deps
```
2. Install the latest version of istio
```
curl -L https://istio.io/downloadIstio | sh -
```

3. Build WASME We use wasme to build, push and deploy our WASM filter.
However, it only supports deploying filters EnvoyFilter_SIDECAR_INBOUND. This WASME code deploys using EnvoyFilter_ANY and it's at taegyunkim/wasme:patch-context
```
cd deps
git clone -b patch-context https://github.com/taegyunkim/wasme.git
mv wasme patched_wasme
cd patched_wasme
make wasme
cd _output
export PATH=$PWD:$PATH
```

4. Use the script below to set up a cluster, enable istio, and deploy the bookinfo application
```
python3 deps/fault_testing.py -s
```
Note:  make sure you have logged into your webassemblyhub.io account by doing "wasme login" before the following step, or it won't work properly
5. Build and push the filter in the messsage_counter directory through
```
python3 fault_testing.py -bf
```
6. Deploy the filter you just built through
```
python3 fault_testing.py -df
```
7. You can print out the $GATEWAY_URL environment variable, and do 
```
curl -v $GATEWAY_URL/productpage
```
to see your running application's information.  In the headers, there should be some extra headers from your filter.

8. Delete the cluster through
```
gcloud container clusters delete demo --zone us-central1-a
```

# To enable Jaeger trace collections
After you have gone through steps 1-3, run
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

