#!/usr/bin/env python3
import argparse
import logging
import sys
import time
import os
import signal
from datetime import datetime
from pathlib import Path

from prometheus_api_client import PrometheusConnect

import util

log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
ROOT_DIR = FILE_DIR.parent
ISTIO_DIR = FILE_DIR.joinpath("istio-1.7.4")
ISTIO_DIR = FILE_DIR.joinpath("istio-1.8.0")
ISTIO_BIN = ISTIO_DIR.joinpath("bin/istioctl")
WASME_BIN = FILE_DIR.joinpath("bin/wasme")
PATCHED_WASME_BIN = FILE_DIR.joinpath("bin/wasme_patched")

#FILTER_DIR = FILE_DIR.joinpath("latency_monitor")
FILTER_DIR = FILE_DIR.joinpath("message_counter")
#FILTER_NAME = "webassemblyhub.io/fruffy/test-filter"
FILTER_NAME = "webassemblyhub.io/jb7399/message-counter"
FILTER_TAG = "1"
FILTER_ID = "test"


# the kubernetes python API sucks

# from kubernetes import client
# from kubernetes.client.configuration import Configuration
# from kubernetes.utils import create_from_yaml
# from kubernetes.config import kube_config
# def get_e2e_configuration():
#     config = Configuration()
#     config.host = None
#     kube_config.load_kube_config(client_configuration=config)
#     print('Running test against : %s' % config.host)
#     return config
# conf = get_e2e_configuration()
# k8s_client = client.api_client.ApiClient(configuration=conf)
# create_from_yaml(k8s_client, f"{bookinfo_dir}/platform/kube/bookinfo.yaml")


def inject_istio():
    cmd = f"{ISTIO_BIN} install --set profile=demo "
    cmd += "--set meshConfig.enableTracing=true --skip-confirmation "
    result = util.exec_process(cmd)
    cmd = "kubectl label namespace default istio-injection=enabled "
    result = util.exec_process(cmd)
    return result


def deploy_addons():
    apply_cmd = "kubectl apply -f "
    url = "https://raw.githubusercontent.com/istio/istio/release-1.7"
    cmd = f"{apply_cmd} {url}/samples/addons/prometheus.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/grafana.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/jaeger.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml || "
    cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml"
    result = util.exec_process(cmd)

    wait_cmd = "/bin/bash -c "
    wait_cmd += "'echo $(kubectl get deploy -n istio-system -o name) |"
    wait_cmd += "{ read -d EOF deploy; for i in $deploy; "
    wait_cmd += "do kubectl rollout status -n istio-system "
    wait_cmd += "$i -w --timeout=180s; done; }'"
    result = util.exec_process(wait_cmd)
    log.info("Addons are ready.")
    return result


def bookinfo_wait():
    wait_cmd = "/bin/bash -c 'echo $(kubectl get deploy -o name) |"
    wait_cmd += "{ read -d EOF deploy; for i in $deploy; "
    wait_cmd += "do kubectl rollout status $i -w --timeout=180s; done; }'"
    result = util.exec_process(wait_cmd)
    log.info("Bookinfo is ready.")
    return result


def deploy_bookinfo():
    # launch bookinfo
    samples_dir = f"{ISTIO_DIR}/samples"
    bookinfo_dir = f"{samples_dir}/bookinfo"
    apply_cmd = f"kubectl apply -f {bookinfo_dir}"
    cmd = f"{apply_cmd}/platform/kube/bookinfo.yaml && "
    cmd += f"{apply_cmd}/networking/bookinfo-gateway.yaml && "
    cmd += f"{apply_cmd}/networking/destination-rule-all.yaml && "
    cmd += f"{apply_cmd}/networking/destination-rule-all-mtls.yaml "
    # cmd += f"{apply_cmd}/../httpbin/sample-client/fortio-deploy.yaml "
    result = util.exec_process(cmd)
    bookinfo_wait()
    cmd2 = """export INGRESS_HOST=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.status.loadBalancer.ingress[0].ip}')"""
    cmd3 = """export INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="http2")].port}')"""
    cmd4 = """export SECURE_INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="https")].port}')"""
    cmd5 = """export TCP_INGRESS_PORT=$(kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="tcp")].port}')"""
    result2 = util.exec_process(cmd2);
    result3 = util.exec_process(cmd3);
    result4 = util.exec_process(cmd4);
    result5 = util.exec_process(cmd5);
    result6 = util.exec_process(f"export GATEWAY_URL=$INGRESS_HOST:$INGRESS_PORT");
    return result, result2, result3, result4, result5


def remove_bookinfo():
    # launch bookinfo
    samples_dir = f"{ISTIO_DIR}/samples"
    bookinfo_dir = f"{samples_dir}/bookinfo"
    cmd = f"{bookinfo_dir}/platform/kube/cleanup.sh "
    result = util.exec_process(cmd)
    return result


def inject_failure():
    cmd = f"kubectl apply -f {FILE_DIR}/fault-injection.yaml "
    result = util.exec_process(cmd)
    return result


def remove_failure():
    cmd = f"kubectl delete -f {FILE_DIR}/fault-injection.yaml "
    result = util.exec_process(cmd)
    return result



def check_pods_status():
    cmd = "kubectl rollout status deployment/"
    deployments = ["details-v1", "productpage-v1", "ratings-v1", "reviews-v1", "reviews-v2", "reviews-v3"]
    deployments_ready = False
    while not deployments_ready:
        all_ready = True
        for deployment in deployments:
            result = util.exec_process(cmd+deployment)
            if "successfully" not in result:
                all_ready = False
        if all_ready == True:
            deployments_ready = True
        else:
            print("Waiting for deployments to stabilize")
            sleep(5)
    return 0

def start_kubernetes():
    cmd = "gcloud container clusters create demo --enable-autoupgrade --enable-autoscaling --min-nodes=3 --max-nodes=10 --num-nodes=5 --zone=us-central1-a"
    result = util.exec_process(cmd)
    return result


def stop_kubernetes():
    # delete minikube
    cmd = "gcloud container clusters delete demo --zone us-central1-a"
    result = util.exec_process(cmd)
    return result


def get_gateway_info():
    cmd = "minikube ip"
    ingress_host = util.get_output_from_proc(cmd).decode("utf-8").rstrip()
    log.info("Ingress Host: %s", ingress_host)
    cmd = "kubectl -n istio-system get service istio-ingressgateway"
    cmd += " -o jsonpath={.spec.ports[?(@.name==\"http2\")].nodePort}"
    ingress_port = util.get_output_from_proc(cmd).decode("utf-8")
    log.info("Ingress Port: %s", ingress_port)
    gateway_url = f"{ingress_host}:{ingress_port}"
    log.info("Gateway: %s", gateway_url)

    return ingress_host, ingress_port, gateway_url


def start_fortio(gateway_url):
    # cmd = "kubectl get pods -lapp=fortio -o jsonpath={.items[0].metadata.name}"
    # fortio_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    # cmd = f"kubectl exec {fortio_pod_name} -c fortio -- /usr/bin/fortio "
    cmd = f"{FILE_DIR}/bin/fortio "
    cmd += "load -c 1 -qps 25 -t 0 -loglevel Warning "
    cmd += f"http://{gateway_url}/productpage"
    fortio_proc = util.start_process(cmd, preexec_fn=os.setsid)
    return fortio_proc


def setup_bookinfo_deployment():
    start_kubernetes()
    inject_istio()
    deploy_bookinfo()
    deploy_addons()


def launch_prometheus():
    if check_kubernetes_status().returncode != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    cmd = "kubectl get pods -n istio-system -lapp=prometheus "
    cmd += " -o jsonpath={.items[0].metadata.name}"
    prom_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    cmd = f"kubectl port-forward -n istio-system {prom_pod_name} 9090"
    prom_proc = util.start_process(cmd, preexec_fn=os.setsid)
    time.sleep(2)
    prom_api = PrometheusConnect(url="http://localhost:9090", disable_ssl=True)

    return prom_proc, prom_api


def query_prometheus(prom_api):
    query = prom_api.custom_query(
        query="(histogram_quantile(0.50, sum(irate(istio_request_duration_milliseconds_bucket{reporter=\"source\",destination_service=~\"ratings.default.svc.cluster.local\"}[1m])) by (le)) / 1000) or histogram_quantile(0.50, sum(irate(istio_request_duration_seconds_bucket{reporter=\"source\",destination_service=~\"ratings.default.svc.cluster.local\"}[1m])) by (le))")
    for q in query:
        val = q["value"]
        query_time = datetime.fromtimestamp(val[0])
        latency = float(val[1]) * 1000
        log.info("Time: %s Latency (ms) %s", query_time, latency)


def query_loop(prom_api, seconds):
    for _ in range(seconds):
        query_prometheus(prom_api)
        time.sleep(1)


def test_fault_injection(prom_api):
    if check_kubernetes_status().returncode != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    # once everything has started, retrieve the necessary url info
    _, _, gateway_url = get_gateway_info()
    fortio_proc = start_fortio(gateway_url)
    # let things sink in a little
    log.info("Running Fortio at time %s", datetime.now())
    query_loop(prom_api, 60)
    log.info("Injecting latency at time %s", datetime.now())
    inject_failure()
    query_loop(prom_api, 60)
    log.info("Removing latency at time %s", datetime.now())
    remove_failure()
    query_loop(prom_api, 60)
    log.info("Done at time %s", datetime.now())
    # terminate fortio by sending an interrupt to the process group
    os.killpg(os.getpgid(fortio_proc.pid), signal.SIGINT)


def build_filter():
    # Bazel is obnoxious, need to explicitly change dirs
    cmd = f"cd {FILTER_DIR}; bazel build //:filter.wasm; cd - "
    result = util.exec_process(cmd)

    cmd = f"{PATCHED_WASME_BIN} build precompiled"
    cmd += f" {FILTER_DIR}/bazel-bin/filter.wasm "
    cmd += f"--tag {FILTER_NAME}:{FILTER_TAG}"
    cmd += f" --config {FILTER_DIR}/runtime-config.json"
    result1 = util.exec_process(cmd)

    print("built filter")
    cmd2 = f"{PATCHED_WASME_BIN} push {FILTER_NAME}:{FILTER_TAG}"
    result2 = util.exec_process(cmd2)
    print("pushed filter")
    return (result1, result2)


def remove_filter():
    cmd = f"{WASME_BIN} undeploy istio {FILTER_NAME}:{FILTER_TAG} "
    cmd += f"–provider=istio --id {FILTER_ID} "
    result = util.exec_process(cmd)
    bookinfo_wait()
    return result


def deploy_filter():
    # first deploy with the unidirectional wasme binary
    cmd = f"{WASME_BIN} deploy istio {FILTER_NAME}:{FILTER_TAG} "
    #cmd += f"-–provider=istio --id {FILTER_ID} "
    cmd += f"--id {FILTER_ID} "
    result = util.exec_process(cmd)
    bookinfo_wait()
    # after we have deployed with the working wasme, remove the deployment
    remove_filter()
    # now redeploy with the patched bidirectional wasme
    cmd = f"{PATCHED_WASME_BIN} deploy istio {FILTER_NAME}:{FILTER_TAG} "
    cmd += f"–provider=istio --id {FILTER_ID} "
    result = util.exec_process(cmd)
    bookinfo_wait()

    return result


def main(args):
    if args.setup:
        return setup_bookinfo_deployment()
    if args.deploy_bookinfo:
        return deploy_bookinfo()
    if args.remove_bookinfo:
        return remove_bookinfo()
    if args.clean:
        return stop_kubernetes()
    if args.build_filter:
        return build_filter()
    if args.deploy_filter:
        return deploy_filter()
    if args.remove_filter:
        return remove_filter()
    if args.full_run:
        setup_bookinfo_deployment()
    # test the fault injection on an existing deployment
    prom_proc, prom_api = launch_prometheus()
    test_fault_injection(prom_api)
    os.killpg(os.getpgid(prom_proc.pid), signal.SIGINT)

    if args.full_run:
        # all done with the test, clean up
        stop_kubernetes()
    return util.EXIT_SUCCESS


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--log-file", dest="log_file",
                        default="model.log",
                        help="Specifies name of the log file.")
    parser.add_argument("-ll", "--log-level", dest="log_level",
                        default="INFO",
                        choices=["CRITICAL", "ERROR", "WARNING",
                                 "INFO", "DEBUG", "NOTSET"],
                        help="The log level to choose.")
    parser.add_argument("-f", "--full-run", dest="full_run",
                        action="store_true",
                        help="Whether to do a full run. "
                        "This includes setting up bookinfo and Kubernetes"
                        " and tearing it down again.")
    parser.add_argument("-s", "--setup", dest="setup",
                        action="store_true",
                        help="Just do a deployment. "
                        "This means installing bookinfo and Kubernetes."
                        " Do not run any experiments.")
    parser.add_argument("-c", "--clean", dest="clean",
                        action="store_true",
                        help="Clean up an existing deployment. ")
    parser.add_argument("-db", "--deploy-bookinfo", dest="deploy_bookinfo",
                        action="store_true",
                        help="Deploy the bookinfo app. ")
    parser.add_argument("-rb", "--remove-bookinfo", dest="remove_bookinfo",
                        action="store_true",
                        help="Remove the bookinfo app. ")
    parser.add_argument("-bf", "--build-filter", dest="build_filter",
                        action="store_true",
                        help="Build the WASME filter. ")
    parser.add_argument("-df", "--deploy-filter", dest="deploy_filter",
                        action="store_true",
                        help="Deploy the WASME filter. ")
    parser.add_argument("-rf", "--remove-filter", dest="remove_filter",
                        action="store_true",
                        help="Remove the WASME filter. ")
    # Parse options and process argv
    arguments = parser.parse_args()
    # configure logging
    logging.basicConfig(filename=arguments.log_file,
                        format="%(levelname)s:%(message)s",
                        level=getattr(logging, arguments.log_level),
                        filemode='w')
    stderr_log = logging.StreamHandler()
    stderr_log.setFormatter(logging.Formatter("%(levelname)s:%(message)s"))
    logging.getLogger().addHandler(stderr_log)
    main(arguments)