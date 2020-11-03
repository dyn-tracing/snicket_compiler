minikube start
#bin/istioctl manifest apply --set profile=demo
# or for newer versions
bin/istioctl install --set profile=demo
kubectl label namespace default istio-injection=enabled
# these are the "patched" applications
kubectl apply -f samples/bookinfo/platform/kube/bookinfo.yaml
kubectl apply -f samples/bookinfo/networking/bookinfo-gateway.yaml
kubectl apply -f samples/bookinfo/networking/destination-rule-all.yaml
kubectl apply -f samples/bookinfo/networking/destination-rule-all-mtls.yaml
./bin/istioctl analyze
export INGRESS_PORT=(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].nodePort}')
export SECURE_INGRESS_PORT=(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="https")].nodePort}')
export INGRESS_HOST=(minikube ip)
export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT
echo http://"$GATEWAY_URL/productpage"
# run on separate shell
minikube tunnelkubectl apply -f wasm/storage_upstream.yaml
kubectl apply -f wasm/productpage-cluster.yaml
target/debug/dyntracing -q example_queries/return.cqlgit clone -b patch-context git@github.com:taegyunkim/wasme.git
cd wasme
make wasme
cd _output
export PATH=$PWD:$PATH
cd ../../cd wasm
bazel build :filter.wasm
../wasme/_output/wasme build precompiled ./bazel-bin/filter.wasm \
                                   --tag webassemblyhub.io/fruffy/testfilter:1  \
                                   --config runtime-config.json
../wasme/_output/wasme push webassemblyhub.io/fruffy/testfilter:1
../wasme/_output/wasme deploy istio webassemblyhub.io/fruffy/testfilter:1 --id=1 --namespace=default
kubectl exec -it (kubectl get pod -l app=ratings -o jsonpath='{.items[0].metadata.name}') -c ratings -- curl -v storage-upstream:8080/list# deletion
samples/bookinfo/platform/kube/cleanup.sh
kubectl delete service/storage-upstream
kubectl delete deployment.apps/storage-upstream
