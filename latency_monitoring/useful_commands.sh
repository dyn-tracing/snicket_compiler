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


########################################################################
# MICROSERVICES DEMO
########################################################################

# start minikube
minikube start

# this injects istio into the cluster
deps/istio-1.7.4/bin/istioctl install
kubectl label namespace default istio-injection=enabled

# set the wasme path in the fish shell
cd wasm
./install_cli.sh
set -gx PATH $HOME/.wasme/bin:$PATH $PATH

# deploy the microservices-demo
# this command might have to be run multiple times because the demo sucks
cd deps/microservices-demo; skaffold run; cd -

# build our filter, replace this with our custom query stuff later
wasme build assemblyscript -t test-filter . \
                                   --tag webassemblyhub.io/fruffy/test-filter:1  \
                                   --config runtime-config.json

wasme push webassemblyhub.io/fruffy/test-filter:1
wasme deploy istio webassemblyhub.io/fruffy/test-filter:1 –provider=istio --id test

# launch the frontend

minikube service frontend-external
export INGRESS_PORT=(kubectl get service frontend-external -o jsonpath='{.spec.ports[*].nodePort}')
export INGRESS_HOST=(minikube ip)
export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT
# get the http headers of the front page
curl -v "http://$GATEWAY_URL"


# The wasm runtime-config should look similiar to this
# {
#   "type": "envoy_proxy",
#   "abiVersions": [
#     "v0-4689a30309abf31aee9ae36e73d34b1bb182685f", <--- newer version of istio
#     "v0-541b2c1155fffb15ccde92b8324f3e38f7339ba6",
#     "v0-097b7f2e4cc1fb490cc1943d0d633655ac3c522f",
#     "v0.2.1"
#   ],
#   "config": {
#     "rootIds": [
#       "add_header"
#     ]
#   }
# }
