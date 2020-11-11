# WARNING: This is not a script that should be run yet
# These are useful commands for setting up and testing a bookinfo cluster

minikube start
#bin/istioctl manifest apply --set profile=demo
# or for newer versions
kubectl create namespace bookinfo
istio-1.7.4/bin/istioctl install --set profile=demo
kubectl label namespace default istio-injection=enabled
# these are the istio applications
kubectl apply -f istio-1.7.4/samples/bookinfo/platform/kube/bookinfo.yaml
kubectl apply -f istio-1.7.4/samples/bookinfo/networking/bookinfo-gateway.yaml
kubectl apply -f istio-1.7.4/samples/bookinfo/networking/destination-rule-all.yaml
kubectl apply -f istio-1.7.4/samples/bookinfo/networking/destination-rule-all-mtls.yaml
kubectl apply -f istio-1.7.4/samples/httpbin/sample-client/fortio-deploy.yaml
istio-1.7.4/bin/istioctl analyze
export INGRESS_PORT=(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].nodePort}')
export INGRESS_HOST=(minikube ip)
export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT
echo http://"$GATEWAY_URL/productpage"

# Fortio
export FORTIO_POD=(kubectl get pods -lapp=fortio -o 'jsonpath={.items[0].metadata.name}')
# execute a single request with fortio
kubectl exec "$FORTIO_POD" -c fortio -- /usr/bin/fortio curl -quiet http://"$GATEWAY_URL/productpage"
# execute 20 requests with fortio
kubectl exec "$FORTIO_POD" -c fortio -- /usr/bin/fortio load -c 2 -qps 0 -n 20 -loglevel Warning http://"$GATEWAY_URL/productpage"

# run on separate shell
minikube tunnel
kubectl apply -f wasm/storage_upstream.yaml
kubectl apply -f wasm/productpage-cluster.yaml
target/debug/dyntracing -q example_queries/return.cql
git clone -b patch-context git@github.com:taegyunkim/wasme.git
cd wasme
make wasme
cd _output
export PATH=$PWD:$PATH
cd ../../
cd wasm
bazel build :filter.wasm
../wasme/_output/wasme build precompiled ./bazel-bin/filter.wasm \
                                   --tag webassemblyhub.io/fruffy/testfilter:1  \
                                   --config runtime-config.json
../wasme/_output/wasme push webassemblyhub.io/fruffy/testfilter:1
../wasme/_output/wasme deploy istio webassemblyhub.io/fruffy/testfilter:1 --id=1 --namespace=default
kubectl exec -it (kubectl get pod -l app=ratings -o jsonpath='{.items[0].metadata.name}') -c ratings -- curl -v storage-upstream:8080/list# deletion

istio-1.7.4/samples/bookinfo/platform/kube/cleanup.sh
kubectl delete service/storage-upstream
kubectl delete deployment.apps/storage-upstream


# deploy addons
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.7/samples/addons/grafana.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.7/samples/addons/prometheus.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.7/samples/addons/kiali.yaml
kubectl apply -f https://raw.githubusercontent.com/istio/istio/release-1.7/samples/addons/jaeger.yaml

# access them
./istio-1.7.4/bin/istioctl dashboard grafana
./istio-1.7.4/bin/istioctl dashboard prometheus
./istio-1.7.4/bin/istioctl dashboard kiali
./istio-1.7.4/bin/istioctl dashboard jaeger
