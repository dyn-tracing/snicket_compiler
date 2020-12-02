#!/usr/bin/env python3
import argparse
import logging
import sys
import time
import os
import signal
from datetime import datetime
from pathlib import Path
import requests

from prometheus_api_client import PrometheusConnect

import util

log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
ROOT_DIR = FILE_DIR.parent
ISTIO_DIR = FILE_DIR.joinpath("istio-1.8.0")
ISTIO_BIN = ISTIO_DIR.joinpath("bin/istioctl")
WASME_BIN = FILE_DIR.joinpath("bin/wasme")
PATCHED_WASME_BIN = FILE_DIR.joinpath("bin/wasme_patched")
YAML_DIR = FILE_DIR.joinpath("yaml_crds")

FILTER_NAME = ""
FILTER_DIR = FILE_DIR.joinpath("message_counter")
FILTER_TAG = "1"
FILTER_ID = "test"


# the kubernetes python API sucks, but keep this for later

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
    if result != util.EXIT_SUCCESS:
        return result
    cmd = "kubectl label namespace default istio-injection=enabled --overwrite"
    result = util.exec_process(cmd)
    return result


def deploy_addons():
    apply_cmd = "kubectl apply -f "
    url = "https://raw.githubusercontent.com/istio/istio/release-1.8"
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
    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    # launch bookinfo
    samples_dir = f"{ISTIO_DIR}/samples"
    bookinfo_dir = f"{samples_dir}/bookinfo"
    apply_cmd = "kubectl apply -f"
    book_cmd = f"{apply_cmd} {bookinfo_dir}"
    cmd = f"{apply_cmd} {YAML_DIR}/bookinfo-mod.yaml && "
    cmd += f"{book_cmd}/networking/bookinfo-gateway.yaml && "
    cmd += f"{book_cmd}/networking/destination-rule-all.yaml && "
    cmd += f"{book_cmd}/networking/destination-rule-all-mtls.yaml && "
    cmd += f"{apply_cmd} {YAML_DIR}/storage-upstream.yaml && "
    cmd += f"{apply_cmd} {YAML_DIR}/productpage-cluster.yaml "
    # disable this and use the non-container version of fortio
    # cmd += f"{apply_cmd}/../httpbin/sample-client/fortio-deploy.yaml "
    result = util.exec_process(cmd)
    bookinfo_wait()
    return result


def remove_bookinfo():
    # launch bookinfo
    samples_dir = f"{ISTIO_DIR}/samples"
    bookinfo_dir = f"{samples_dir}/bookinfo"
    cmd = f"{bookinfo_dir}/platform/kube/cleanup.sh &&"
    cmd += f"kubectl delete -f {YAML_DIR}/storage-upstream.yaml && "
    cmd += f"kubectl delete -f {YAML_DIR}/productpage-cluster.yaml "
    result = util.exec_process(cmd)
    return result


def inject_failure():
    cmd = f"kubectl apply -f {YAML_DIR}/fault-injection.yaml "
    result = util.exec_process(cmd)
    return result


def remove_failure():
    cmd = f"kubectl delete -f {YAML_DIR}/fault-injection.yaml "
    result = util.exec_process(cmd)
    return result


def check_kubernetes_status():
    cmd = "minikube status"
    result = util.exec_process(cmd)
    return result


def start_kubernetes(platform):
    if platform == "GCP":
        cmd = "gcloud container clusters create demo --enable-autoupgrade "
        cmd += "--enable-autoscaling --min-nodes=3 --max-nodes=10 "
        cmd += "--num-nodes=5 --zone=us-central1-a"
    else:
        cmd = "minikube start --memory=8192 --cpus=4 "
    result = util.exec_process(cmd)
    return result


def stop_kubernetes(platform):
    if platform == "GCP":
        cmd = "gcloud container clusters delete demo --zone us-central1-a"
    else:
        # delete minikube
        cmd = "minikube delete"
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


def setup_bookinfo_deployment(platform):
    start_kubernetes(platform)
    result = inject_istio()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_bookinfo()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_addons()
    return result


def launch_prometheus():
    if check_kubernetes_status() != util.EXIT_SUCCESS:
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


def launch_storage_mon():
    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    cmd = "kubectl get pods -lapp=storage-upstream "
    cmd += " -o jsonpath={.items[0].metadata.name}"
    storage_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    cmd = f"kubectl port-forward {storage_pod_name} 8090"
    storage_proc = util.start_process(cmd, preexec_fn=os.setsid)
    time.sleep(2)

    return storage_proc


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
    if check_kubernetes_status() != util.EXIT_SUCCESS:
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


def build_filter(filter_dir, filter_name):
    # Bazel is obnoxious, need to explicitly change dirs
    log.info("Building filter...")
    cmd = f"cd {filter_dir}; bazel build //:filter.wasm"
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        return result

    cmd = f"{PATCHED_WASME_BIN} build precompiled"
    cmd += f" {filter_dir}/bazel-bin/filter.wasm "
    cmd += f"--tag {filter_name}:{FILTER_TAG}"
    cmd += f" --config {filter_dir}/runtime-config.json"
    result = util.exec_process(cmd)
    log.info("Done with building filter...")
    if result != util.EXIT_SUCCESS:
        return result

    log.info("Pushing the filter...")
    cmd = f"{PATCHED_WASME_BIN} push {filter_name}:{FILTER_TAG}"
    result = util.exec_process(cmd)
    # Give it some room to breathe
    time.sleep(2)
    return result


def undeploy_filter(filter_name):
    cmd = f"kubectl delete -f {YAML_DIR}/istio-config.yaml && "
    cmd += f"kubectl delete -f {YAML_DIR}/virtual-service-reviews-balance.yaml "
    util.exec_process(cmd)
    cmd = f"{WASME_BIN} undeploy istio {filter_name}:{FILTER_TAG} "
    cmd += f"–provider=istio --id {FILTER_ID} "
    cmd += "--labels \"app=reviews\" "
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        return result
    bookinfo_wait()
    return result


def deploy_filter(filter_name):
    # first deploy with the unidirectional wasme binary
    cmd = f"{WASME_BIN} deploy istio {filter_name}:{FILTER_TAG} "
    cmd += f"–provider=istio --id {FILTER_ID} "
    cmd += "--labels \"app=reviews\" "
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        return result
    bookinfo_wait()
    # after we have deployed with the working wasme, remove the deployment
    undeploy_filter(filter_name)
    # now redeploy with the patched bidirectional wasme
    cmd = f"{PATCHED_WASME_BIN} deploy istio {filter_name}:{FILTER_TAG} "
    cmd += f"–provider=istio --id {FILTER_ID} "
    cmd += "--labels \"app=reviews\" "
    result = util.exec_process(cmd)
    bookinfo_wait()
    # apply our customization configuration to the mesh
    cmd = f"kubectl apply -f {YAML_DIR}/istio-config.yaml && "
    cmd += f"kubectl apply -f {YAML_DIR}/virtual-service-reviews-balance.yaml "
    result = util.exec_process(cmd)

    return result


def refresh_filter(filter_dir, filter_name):
    result = build_filter(filter_dir, filter_name)
    if result != util.EXIT_SUCCESS:
        return result
    result = undeploy_filter(filter_name)
    if result != util.EXIT_SUCCESS:
        # A failure here is actually okay
        log.warning("Undeploying failed.")
    result = deploy_filter(filter_name)
    return result


def handle_filter(args):
    is_filter_related = args.build_filter or args.deploy_filter
    is_filter_related = is_filter_related or args.undeploy_filter
    is_filter_related = is_filter_related or args.refresh_filter
    if not is_filter_related:
        return
    if not args.filter_name:
        log.error("The filter name is required to deploy filters with wasme. "
                  "You can set it with the -fn or --filter-name argument.")
        sys.exit(util.EXIT_FAILURE)
    if args.build_filter:
        result = build_filter(args.filter_dir, args.filter_name)
        sys.exit(result)
    if args.deploy_filter:
        result = deploy_filter(args.filter_name)
        sys.exit(result)
    if args.undeploy_filter:
        result = undeploy_filter(args.filter_name)
        sys.exit(result)
    if args.refresh_filter:
        result = refresh_filter(args.filter_dir, args.filter_name)
        sys.exit(result)


def main(args):
    if args.setup:
        return setup_bookinfo_deployment(args.platform)
    if args.deploy_bookinfo:
        return deploy_bookinfo()
    if args.remove_bookinfo:
        return remove_bookinfo()
    handle_filter(args)
    if args.clean:
        return stop_kubernetes(args.platform)
    if args.full_run:
        setup_bookinfo_deployment(args.platform)
    # test the fault injection on an existing deployment
    prom_proc, prom_api = launch_prometheus()
    test_fault_injection(prom_api)
    os.killpg(os.getpgid(prom_proc.pid), signal.SIGINT)

    if args.full_run:
        # all done with the test, clean up
        stop_kubernetes(args.platform)
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
    parser.add_argument("-p", "--platform", dest="platform",
                        default="KB",
                        choices=["MK", "GCP"],
                        help="Which platform to run the scripts on."
                             "MK is minikube, GCP is Google Cloud Compute")
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
    parser.add_argument("-fn", "--filter-name", dest="filter_name",
                        default=FILTER_NAME,
                        help="The name of the filter to push to the Wasm Hub.")
    parser.add_argument("-fd", "--filter-dir", dest="filter_dir",
                        default=FILTER_DIR,
                        help="The directory of the filter")
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
    parser.add_argument("-uf", "--undeploy-filter", dest="undeploy_filter",
                        action="store_true",
                        help="Remove the WASME filter. ")
    parser.add_argument("-rf", "--refresh-filter", dest="refresh_filter",
                        action="store_true",
                        help="Refresh the WASME filter. ")
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
