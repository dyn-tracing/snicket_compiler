#!/usr/bin/env python3
import argparse
import logging
import sys
import time
import os
import signal

from pathlib import Path

import util

log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
ROOT_DIR = FILE_DIR.parent
ISTIO_DIR = FILE_DIR.joinpath("istio-1.7.4")
ISTIO_BIN = ISTIO_DIR.joinpath("bin/istioctl")

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
    cmd = f"{ISTIO_BIN} install --set profile=demo"
    result = util.exec_process(cmd)
    cmd = "kubectl label namespace default istio-injection=enabled"
    result = util.exec_process(cmd)
    return result


def deploy_addons():
    apply_cmd = f"kubectl apply -f "
    url = "https://raw.githubusercontent.com/istio/istio/release-1.7"
    cmd = f"{apply_cmd} {url}/samples/addons/prometheus.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/grafana.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/jaeger.yaml && "
    cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml || "
    cmd += f"{apply_cmd} {url}/samples/addons/kiali.yaml"
    result = util.exec_process(cmd)
    return result


def deploy_bookinfo():
    # launch bookinfo
    samples_dir = f"{ISTIO_DIR}/samples"
    bookinfo_dir = f"{samples_dir}/bookinfo"
    apply_cmd = f"kubectl apply -f {bookinfo_dir}"
    cmd = f"{apply_cmd}/platform/kube/bookinfo.yaml && "
    cmd += f"{apply_cmd}/networking/bookinfo-gateway.yaml && "
    cmd += f"{apply_cmd}/networking/destination-rule-all.yaml && "
    cmd += f"{apply_cmd}/networking/destination-rule-all-mtls.yaml && "
    cmd += f"{apply_cmd}/../httpbin/sample-client/fortio-deploy.yaml "
    result = util.exec_process(cmd)

    wait_cmd = "/bin/bash -c 'echo $(kubectl get deploy -o name) |"
    wait_cmd += "{ read -d EOF deploy; for i in $deploy; "
    wait_cmd += "do kubectl rollout status $i -w --timeout=180s; done; }'"
    result = util.exec_process(wait_cmd)
    log.info("Bookinfo is ready.")
    return result


def inject_failure():
    cmd = "kubectl apply -f fault-injection.yaml "
    result = util.exec_process(cmd)
    return result


def remove_failure():
    cmd = "kubectl delete -f fault-injection.yaml "
    result = util.exec_process(cmd)
    return result


def check_kubernetes_status():
    cmd = "minikube status"
    result = util.exec_process(cmd)
    return result


def start_kubernetes():
    cmd = "minikube start"
    result = util.exec_process(cmd)
    return result


def stop_kubernetes():
    # delete minikube
    cmd = "minikube delete"
    result = util.exec_process(cmd)
    return result


def get_gateway_info():
    cmd = "minikube ip"
    ingress_host = util.get_output_from_proc(cmd).decode("utf-8").rstrip()
    log.info("Ingress Host: %s", ingress_host)
    cmd = "kubectl -n istio-system get service istio-ingressgateway -o jsonpath={.spec.ports[?(@.name==\"http2\")].nodePort}"
    ingress_port = util.get_output_from_proc(cmd).decode("utf-8")
    log.info("Ingress Port: %s", ingress_port)
    gateway_url = f"{ingress_host}:{ingress_port}"
    log.info("Gateway: %s", gateway_url)

    return ingress_host, ingress_port, gateway_url


def start_fortio(gateway_url):
    # cmd = "kubectl get pods -lapp=fortio -o jsonpath={.items[0].metadata.name}"
    # fortio_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    # cmd = f"kubectl exec {fortio_pod_name} -c fortio -- /usr/bin/fortio "
    cmd = f"{FILE_DIR}/fortio "
    cmd += "load -c 1 -qps 25 -t 0 -loglevel Warning "
    cmd += f"http://{gateway_url}/productpage"
    fortio_proc = util.start_process(cmd, preexec_fn=os.setsid)
    return fortio_proc


def setup_bookinfo_deployment():
    start_kubernetes()
    inject_istio()
    deploy_bookinfo()
    deploy_addons()


def test_fault_injection():
    if check_kubernetes_status().returncode != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    # once everything has started, retrieve the necessary url info
    _, _, gateway_url = get_gateway_info()
    fortio_proc = start_fortio(gateway_url)
    # let things sink in a little
    log.info("Running Fortio for 20 seconds...")
    time.sleep(20)
    log.info("Injecting latency")
    inject_failure()
    time.sleep(20)
    log.info("Removing latency")
    remove_failure()
    log.info("Done")
    # terminate fortio by sending an interrupt to the process group
    os.killpg(os.getpgid(fortio_proc.pid), signal.SIGINT)


def main(args):
    if args.full_run or args.setup:
        setup_bookinfo_deployment()
    # test the fault injection on an existing deployment
    if not args.setup:
        test_fault_injection()
    if args.full_run:
        # all done with the test, clean up
        stop_kubernetes()


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
