#!/usr/bin/env python3
import argparse
import logging
import sys
import os
from multiprocessing import Process
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import requests

import util

log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
ROOT_DIR = FILE_DIR.parent
ISTIO_DIR = FILE_DIR.joinpath("istio-1.8.0")
ISTIO_BIN = ISTIO_DIR.joinpath("bin/istioctl")
YAML_DIR = FILE_DIR.joinpath("yaml_crds")
TOOLS_DIR = FILE_DIR.joinpath("tools")

FILTER_DIR = FILE_DIR.joinpath("../cpp_filter")

# the kubernetes python API sucks, but keep this for later

# from kubernetes import client
# from kubernetes.client.configuration import Configuration
# from kubernetes.utils import create_from_yaml
# from kubernetes.config import kube_config
# def get_e2e_configuration():
#     config = Configuration()
#     config.host = None
#     kube_config.load_kube_config(client_configuration=config)
#     log.info('Running test against : %s' % config.host)
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
    cmd = f"{apply_cmd} {YAML_DIR}/prometheus-mod.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/grafana.yaml "
    # cmd += f"{apply_cmd} {url}/samples/addons/jaeger.yaml && "
    # cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml || "
    # cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml"
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        return result

    cmd = "kubectl get deploy -n istio-system -o name"
    deployments = util.get_output_from_proc(cmd).decode("utf-8").strip()
    deployments = deployments.split("\n")
    for depl in deployments:
        wait_cmd = "kubectl rollout status -n istio-system "
        wait_cmd += f"{depl} -w --timeout=180s"
        _ = util.exec_process(wait_cmd)
    log.info("Addons are ready.")
    return util.EXIT_SUCCESS


def bookinfo_wait():
    cmd = "kubectl get deploy -o name"
    deployments = util.get_output_from_proc(cmd).decode("utf-8").strip()
    deployments = deployments.split("\n")
    for depl in deployments:
        wait_cmd = f"kubectl rollout status {depl} -w --timeout=180s"
        _ = util.exec_process(wait_cmd)
    log.info("Bookinfo is ready.")
    return util.EXIT_SUCCESS


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
    cmd += f"{book_cmd}/networking/destination-rule-reviews.yaml && "
    cmd += f"{apply_cmd} {YAML_DIR}/storage-upstream.yaml && "
    cmd += f"{apply_cmd} {YAML_DIR}/productpage-cluster.yaml "
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
    cmd = "kubectl cluster-info"
    result = util.exec_process(
        cmd, stdout=util.subprocess.PIPE, stderr=util.subprocess.PIPE)
    return result


def start_kubernetes(platform, multizonal):
    if platform == "GCP":
        cmd = "gcloud container clusters create demo --enable-autoupgrade "
        cmd += "--enable-autoscaling --min-nodes=3 "
        cmd += "--max-nodes=10 --num-nodes=5 "
        if multizonal:
            cmd += "--region us-central1-a --node-locations us-central1-b "
            cmd += "us-central1-c us-central1-a "
        else:
            cmd += "--zone=us-central1-a "
    else:
        cmd = "minikube start --memory=8192 --cpus=4 "
    result = util.exec_process(cmd)
    return result


def stop_kubernetes(platform):
    if platform == "GCP":
        cmd = "gcloud container clusters delete "
        cmd += "demo --zone us-central1-a --quiet "
    else:
        # delete minikube
        cmd = "minikube delete"
    result = util.exec_process(cmd)
    return result


def get_gateway_info(platform):
    ingress_host = ""
    ingress_port = ""
    if platform == "GCP":
        cmd = "kubectl -n istio-system get service istio-ingressgateway "
        cmd += "-o jsonpath={.status.loadBalancer.ingress[0].ip} "
        ingress_host = util.get_output_from_proc(
            cmd).decode("utf-8").replace("'", "")

        cmd = "kubectl -n istio-system get service istio-ingressgateway "
        cmd += " -o jsonpath={.spec.ports[?(@.name==\"http2\")].port}"
        ingress_port = util.get_output_from_proc(
            cmd).decode("utf-8").replace("'", "")
    else:
        cmd = "minikube ip"
        ingress_host = util.get_output_from_proc(cmd).decode("utf-8").rstrip()
        cmd = "kubectl -n istio-system get service istio-ingressgateway"
        cmd += " -o jsonpath={.spec.ports[?(@.name==\"http2\")].nodePort}"
        ingress_port = util.get_output_from_proc(cmd).decode("utf-8")

    log.debug("Ingress Host: %s", ingress_host)
    log.debug("Ingress Port: %s", ingress_port)
    gateway_url = f"{ingress_host}:{ingress_port}"
    log.debug("Gateway: %s", gateway_url)

    return ingress_host, ingress_port, gateway_url


def burst_loop(url):
    NUM_REQUESTS = 500
    MAX_THREADS = 32

    def timeout_request(_):
        try:
            # the timeout effectively makes this request async
            requests.get(url, timeout=0.001)
        except requests.exceptions.ReadTimeout:
            pass

    log.info("Starting burst...")
    # quick hack until I found a better way
    with ThreadPoolExecutor(max_workers=MAX_THREADS) as p:
        for _ in p.map(timeout_request, range(NUM_REQUESTS)):
            pass
    log.info("Done with burst...")


def do_burst(platform):
    _, _, gateway_url = get_gateway_info(platform)
    url = f"http://{gateway_url}/productpage"
    p = Process(target=burst_loop, args=(url, ))
    p.start()
    # do not care about killing that process


def start_fortio(gateway_url):
    cmd = f"{FILE_DIR}/bin/fortio "
    cmd += "load -c 50 -qps 300 -jitter -t 0 -loglevel Warning "
    cmd += f"http://{gateway_url}/productpage"
    fortio_proc = util.start_process(cmd, preexec_fn=os.setsid)
    return fortio_proc


def setup_bookinfo_deployment(platform, multizonal):
    start_kubernetes(platform, multizonal)
    result = inject_istio()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_bookinfo()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_addons()
    return result


def build_filter(filter_dir):
    # Bazel is obnoxious, need to explicitly change dirs
    log.info("Building filter...")
    cmd = f"cd {filter_dir}; bazel build //:filter.wasm"
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        return result

    return result


def undeploy_filter():
    cmd = f"kubectl delete -f {YAML_DIR}/filter.yaml "
    result = util.exec_process(cmd)
    return result


def deploy_filter(filter_dir):
    # check if the config map already exists
    cmd = "kubectl get configmaps example-filter "
    result = util.exec_process(cmd)
    if result != util.EXIT_SUCCESS:
        # create the config map with the filter
        cmd = "kubectl create configmap example-filter --from-file "
        cmd += f"{filter_dir}/bazel-bin/filter.wasm --dry-run=client -o yaml "
        cmd += "| kubectl apply -f -"
        result = util.exec_process(cmd)
        if result != util.EXIT_SUCCESS:
            return result
    else:
        log.info("Config map example-filter already exists!")
    # update the containers with the config map
    cmd = f"kubectl replace -f {YAML_DIR}/bookinfo-mod-filter.yaml "
    # FIXME: There is an issue with the yaml currently, so we ignore the result
    _ = util.exec_process(cmd)
    result = bookinfo_wait()
    if result != util.EXIT_SUCCESS:
        return result
    # now activate the filter
    cmd = f"kubectl apply -f {YAML_DIR}/filter.yaml "
    result = util.exec_process(cmd)
    return result


def refresh_filter():
    # this is equivalent to a deployment restart right now
    cmd = "kubectl rollout restart  deployments --namespace=default"
    result = util.exec_process(cmd)
    return result


def handle_filter(args):
    if args.build_filter:
        return build_filter(args.filter_dir)
    if args.deploy_filter:
        return deploy_filter(args.filter_dir)
    if args.undeploy_filter:
        return undeploy_filter()
    if args.refresh_filter:
        return refresh_filter()
    log.warning("No command line input provided. Do nothing.")
    return util.EXIT_SUCCESS


def main(args):
    # single commands to execute
    if args.setup:
        return setup_bookinfo_deployment(args.platform, args.multizonal)
    if args.deploy_bookinfo:
        return deploy_bookinfo()
    if args.remove_bookinfo:
        return remove_bookinfo()
    if args.clean:
        return stop_kubernetes(args.platform)
    if args.burst:
        return do_burst(args.platform)
    return handle_filter(args)


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
    parser.add_argument("-m", "--multi-zonal", dest="multizonal",
                        action="store_true",
                        help="If you are running on GCP,"
                        " do you want a multi-zone cluster?")
    parser.add_argument("-s", "--setup", dest="setup",
                        action="store_true",
                        help="Just do a deployment. "
                        "This means installing bookinfo and Kubernetes."
                        " Do not run any experiments.")
    parser.add_argument("-c", "--clean", dest="clean",
                        action="store_true",
                        help="Clean up an existing deployment. ")
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
                        help="Build the WASM filter. ")
    parser.add_argument("-df", "--deploy-filter", dest="deploy_filter",
                        action="store_true",
                        help="Deploy the WASM filter. ")
    parser.add_argument("-uf", "--undeploy-filter", dest="undeploy_filter",
                        action="store_true",
                        help="Remove the WASM filter. ")
    parser.add_argument("-rf", "--refresh-filter", dest="refresh_filter",
                        action="store_true",
                        help="Refresh the WASM filter. ")
    parser.add_argument("-b", "--burst", dest="burst",
                        action="store_true",
                        help="Burst with HTTP requests to cause"
                        " congestion and queue buildup.")
    # Parse options and process argv
    arguments = parser.parse_args()
    # configure logging
    logging.basicConfig(filename=arguments.log_file,
                        format="%(levelname)s:%(message)s",
                        level=getattr(logging, arguments.log_level),
                        filemode="w")
    stderr_log = logging.StreamHandler()
    stderr_log.setFormatter(logging.Formatter("%(levelname)s:%(message)s"))
    logging.getLogger().addHandler(stderr_log)
    main(arguments)
