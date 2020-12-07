#!/usr/bin/env python3
import argparse
import logging
import sys
import time
import os
import signal
import csv
from datetime import datetime
from pathlib import Path
import requests
from multiprocessing import Process

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
TOOLS_DIR = FILE_DIR.joinpath("tools")

FILTER_NAME = ""
FILTER_DIR = FILE_DIR.joinpath("message_counter")
FILTER_TAG = "1"
FILTER_ID = "test"
CONGESTION_PERIOD = 1607016396875512000
TO_NANOSECONDS = 1e9  # everything is in nanoseconds
OUTPUT_FILE = "output.csv"
NUM_EXPERIMENTS = 10

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
    cmd = "kubectl cluster-info"
    result = util.exec_process(cmd)
    return result


def start_kubernetes(platform, multizonal):
    if platform == "GCP":
        if multizonal:
            cmd = "gcloud container clusters create demo --enable-autoupgrade "
            cmd += "--enable-autoscaling --min-nodes=3 --max-nodes=10 "
            cmd += "--num-nodes=5 --region us-central1-a --node-locations "
            cmd += "us-central1-b us-central1-c us-central1-a "
        else:
            cmd = "gcloud container clusters create demo --enable-autoupgrade "
            cmd += "--enable-autoscaling --min-nodes=3 --max-nodes=10 "
            cmd += "--num-nodes=5 --zone=us-central1-a "
    else:
        cmd = "minikube start --memory=8192 --cpus=4 "
    result = util.exec_process(cmd)
    return result


def stop_kubernetes(platform):
    if platform == "GCP":
        cmd = "gcloud container clusters delete demo --zone us-central1-a --quiet"
    else:
        # delete minikube
        cmd = "minikube delete"
    result = util.exec_process(cmd)
    return result


def get_gateway_info(platform):
    ingress_host = ""
    ingress_port = ""
    if platform == "GCP":
        cmd = f"kubectl -n istio-system get service istio-ingressgateway -o jsonpath=" + "'"
        cmd += "{.status.loadBalancer.ingress[0].ip}" + "'"
        ingress_host = util.get_output_from_proc(
            cmd).decode("utf-8").replace("'", "")

        cmd = "kubectl -n istio-system get service istio-ingressgateway -o jsonpath='{.spec.ports[?(@.name=="
        cmd += '"' + "http2" + '"' + ")].port}'"
        ingress_port = util.get_output_from_proc(
            cmd).decode("utf-8").replace("'", "")
    else:
        cmd = "minikube ip"
        ingress_host = util.get_output_from_proc(cmd).decode("utf-8").rstrip()
        cmd = "kubectl -n istio-system get service istio-ingressgateway"
        cmd += " -o jsonpath={.spec.ports[?(@.name==\"http2\")].nodePort}"
        ingress_port = util.get_output_from_proc(cmd).decode("utf-8")

    log.info("Ingress Host: %s", ingress_host)
    log.info("Ingress Port: %s", ingress_port)
    gateway_url = f"{ingress_host}:{ingress_port}"
    log.info("Gateway: %s", gateway_url)

    return ingress_host, ingress_port, gateway_url


def start_fortio(gateway_url):
    # cmd = "kubectl get pods -lapp=fortio -o jsonpath={.items[0].metadata.name}"
    # fortio_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    # cmd = f"kubectl exec {fortio_pod_name} -c fortio -- /usr/bin/fortio "
    cmd = f"{FILE_DIR}/bin/fortio "
    cmd += "load -c 50 -qps 2000 -jitter -t 0 -loglevel Warning "
    cmd += f"http://{gateway_url}/productpage"
    fortio_proc = util.start_process(cmd, preexec_fn=os.setsid)
    return fortio_proc


def wait_until_pods_ready(platform):
    if platform == "GCP":
        cmd = "kubectl get pods --field-selector status.phase!=Running --all-namespaces"
        output = util.get_output_from_proc(cmd).decode("utf-8")
        while "No resources found" not in str(output) and output != "" and output != b'':
            log.info("waiting for resources to run")
            time.sleep(2)
            cmd = "kubectl get pods --field-selector status.phase!=Running --all-namespaces"
            output = util.get_output_from_proc(cmd)


def setup_bookinfo_deployment(platform, multizonal):
    start_kubernetes(platform, multizonal)
    result = inject_istio()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_bookinfo()
    if result != util.EXIT_SUCCESS:
        return result
    result = deploy_addons()
    wait_until_pods_ready(platform)
    return result


def cause_congestion(platform):
    _, _, gateway_url = get_gateway_info(platform)
    cmd = f"{TOOLS_DIR}/parallel_curl/pc {gateway_url}/productpage"
    result = util.exec_process(
        cmd, stdout=util.subprocess.PIPE, stderr=util.subprocess.PIPE)
    return result


def find_congestion(platform, starting_time):
    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    logs = query_storage(platform)
    if logs == []:
        return None
    if not logs:
        log.info("No congestion found!")
        return None
    # we want to make sure we aren't recording anything earlier
    # than our starting time. That wouldn't make sense
    ival_start = -1
    ival_end = -1
    for idx, (time_stamp, _) in enumerate(logs):
        if int(time_stamp) > starting_time:
            ival_start = idx
            break
    for idx, (time_stamp, _) in enumerate(logs):
        if int(time_stamp) > (starting_time + CONGESTION_PERIOD):
            ival_end = idx
            break

    congestion_dict = {}
    for idx, (time_stamp, service_name) in enumerate(logs[ival_start:ival_end]):
        congestion_dict[service_name] = int(time_stamp)
        # we have congestion at more than 1 service
        if len(congestion_dict) > 1:
            for congested_service, congestion_ts in congestion_dict.items():
                congestion_ts_str = ns_to_timestamp(congestion_ts)
                log_str = (
                    f"Congestion at {congested_service} at time {congestion_ts_str}")
                log.info(log_str)
            return min(congestion_dict.values())

    log.info("No congestion found")
    return None


def query_storage(platform):
    if platform == "GCP":
        time.sleep(10)  # wait for logs to come in
        logs = []
        cmd = f"{TOOLS_DIR}/logs_script.sh"
        output = util.get_output_from_proc(cmd).decode("utf-8").split("\n")
        for line in output:
            if "Stored" in line:
                line = line[line.find("Stored"):]  # get right after timestamp
                line = line.split()
                timestamp = line[1]
                name = line[-1]
                logs.append([timestamp, name])
    else:
        storage_content = requests.get("http://localhost:8090/list")
        output = storage_content.text.split("\n")
        logs = []
        for line in output:
            if "->" in line:
                line_time, line_name = line.split("->")
                logs.append([line_time, line_name])
    return sorted(logs)


def ns_to_timestamp(str_ns):
    ns = int(str_ns)
    dt = datetime.fromtimestamp(ns / 1e9)
    return f"{dt.strftime('%H:%M:%S.%f')}.{(ns % 1e3):03.0f}"


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
    cmd = f"kubectl port-forward {storage_pod_name} 8090:8080"
    storage_proc = util.start_process(cmd, preexec_fn=os.setsid)
    # Let settle things in a bit
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

def query_csv_loop(prom_api):
    with open("prom.csv", "w+") as csvfile:
        writer = csv.writer(csvfile, delimiter=',')
        writer.writerow(["Time", "RPS"])
        while True:
            query = prom_api.custom_query(
                query="(histogram_quantile(0.90, sum(irate(istio_request_duration_milliseconds_bucket{reporter=\"source\",destination_service=~\"productpage.default.svc.cluster.local\"}[1m])) by (le)) / 1000) or histogram_quantile(0.90, sum(irate(istio_request_duration_seconds_bucket{reporter=\"source\",destination_service=~\"productpage.default.svc.cluster.local\"}[1m])) by (le))")
            for q in query:
                val = q["value"]
                query_time = "{:.7f}".format(val[0]*TO_NANOSECONDS)
                rps = val[1]
                log.info("Time: %s Requests per second %s", query_time, rps)
                writer.writerow([query_time, rps])
                query_time = datetime.fromtimestamp(val[0])
                latency = float(val[1]) * 1000
                log.info("Time: %s 90pct Latency (ms) %s", query_time, latency)
                writer.writerow([query_time, latency])
                csvfile.flush()
            time.sleep(1)


def test_fault_injection(prom_api, platform):
    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    # once everything has started, retrieve the necessary url info
    _, _, gateway_url = get_gateway_info(platform)
    cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
    log.info("Running Fortio at time %s", cur_time)
    fortio_proc = start_fortio(gateway_url)
    # let things sink in a little
    query_loop(prom_api, 30)
    cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
    log.info("Injecting latency at time %s", cur_time)
    inject_failure()
    query_loop(prom_api, 30)
    cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
    log.info("Removing latency at time %s", cur_time)
    remove_failure()
    # query_loop(prom_api, 60)
    cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
    log.info("Done at time %s", cur_time)
    # terminate fortio by sending an interrupt to the process group
    os.killpg(os.getpgid(fortio_proc.pid), signal.SIGINT)

    log.info("Cluster killed")


def do_multiple_runs(platform, num_runs, output_file):
    with open(output_file, "w+") as csvfile:
        writer = csv.writer(csvfile, delimiter=' ')
        writer.writerow(["found congestion?", "congestion started",
                         "congested detected",
                         "difference in nanoseconds",
                         "difference in seconds", "average load between inducing and detecting congestion"])
        for _ in range(int(num_runs)):
            time.sleep(30)
            # once everything has started, retrieve the necessary url info
            cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
            log.info("Injecting latency at time %s", cur_time)
            inject_failure()
            time.sleep(10)
            log.info("Sending burst")
            time_of_congestion = time.time() * TO_NANOSECONDS
            log.info("Causing congestion at %s", time_of_congestion)
            cause_congestion(platform)
            time.sleep(10)
            cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
            log.info("Removing latency at time %s", cur_time)
            remove_failure()
            time.sleep(30)
            cur_time = ns_to_timestamp(time.time() * TO_NANOSECONDS)
            log.info("Done at time %s", cur_time)
            first_recorded_congestion = find_congestion(
                platform, time_of_congestion)
            if first_recorded_congestion is not None:
                congestion_ts = ns_to_timestamp(time_of_congestion)
                detection_ts = ns_to_timestamp(first_recorded_congestion)
                log.info("Sent burst at %s recorded it at %s",
                         congestion_ts, detection_ts)
                latency = (int(first_recorded_congestion) -
                           int(time_of_congestion))
                log.info(
                    "Latency between sending and recording in storage is %s seconds", (latency / TO_NANOSECONDS))
                with open("prom.csv", "r") as prom:
                    avg = 0
                    num_of_recordings = 0.0
                    read = csv.reader(prom)
                    for line in read:
                        if line[0] != "Time": # don't look at the header
                            timestamp = float(line[0])
                            if timestamp > time_of_congestion and timestamp < first_recorded_congestion:
                                avg += float(line[1])
                                num_of_recordings += 1
                    avg = avg/num_of_recordings
                    writer.writerow(
                        ["yes", time_of_congestion, first_recorded_congestion, latency, (latency / TO_NANOSECONDS), avg])
            else:
                writer.writerow(["no", "." * 5])
                log.info("No congestion caused")
            # sleep long enough that the congestion times will not be mixed up
            time.sleep(5)


def do_experiment(platform, multizonal, filter_name, num_experiments, output_file):
    """
    setup_bookinfo_deployment(platform, multizonal)
    wait_until_pods_ready(platform)
    # prom_proc, prom_api = launch_prometheus()
    log.info("deploying filter")
    deploy_filter(filter_name)
    wait_until_pods_ready(platform)
    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)

    if check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)

    # clean up any proc listening on 9090 just to be safe
    cmd = "lsof -ti tcp:9090 | xargs kill || exit 0"
    _ = util.exec_process(
        cmd, stdout=util.subprocess.PIPE, stderr=util.subprocess.PIPE)

    if platform != "GCP":
        # clean up any proc listening on 8090 just to be safe
        cmd = "lsof -ti tcp:8090 | xargs kill || exit 0"
        _ = util.exec_process(
            cmd, stdout=util.subprocess.PIPE, stderr=util.subprocess.PIPE)
    else:
        setup_bookinfo_deployment(platform, multizonal)
        wait_until_pods_ready(platform)
        # prom_proc, prom_api = launch_prometheus()
        log.info("Deploying filter")
        deploy_filter(filter_name)
        wait_until_pods_ready(platform)

    """
    prom_proc, prom_api = launch_prometheus()
    time.sleep(10)
    p = Process(target=query_csv_loop, args=(prom_api, ))
    p.start()
    do_multiple_runs(platform, num_experiments, output_file)
    log.info("Killing fortio")
    # terminate fortio by sending an interrupt to the process group
    os.killpg(os.getpgid(fortio_proc.pid), signal.SIGINT)
    # kill the storage proc after the query
    log.info("Killing storage")
    os.killpg(os.getpgid(storage_proc.pid), signal.SIGINT)
    # kill prometheus
    log.info("Killing prometheus.")
    p.terminate()
    os.killpg(os.getpgid(prom_proc.pid), signal.SIGINT)


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
        return setup_bookinfo_deployment(args.platform, args.multizonal)
    if args.deploy_bookinfo:
        return deploy_bookinfo()
    if args.remove_bookinfo:
        return remove_bookinfo()
    handle_filter(args)
    if args.clean:
        return stop_kubernetes(args.platform)
    if args.full_run:
        setup_bookinfo_deployment(args.platform, args.multizonal)
    if args.cause_congestion:
        return cause_congestion(args.platform)
    # test the fault injection on an existing deployment
    do_experiment(args.platform, args.multizonal, args.filter_name,
                  args.num_experiments, args.output_file)
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
    parser.add_argument("-m", "--multi-zonal", dest="multizonal",
                        action="store_true",
                        help="If you are running on GCP, do you want a multi-zone cluster?")
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
    parser.add_argument("-fc", "--find-congestion", dest="find_congestion",
                        action="store_true",
                        help="Find congestion in the logs. ")
    parser.add_argument("-cc", "--cause-congestion", dest="cause_congestion",
                        action="store_true",
                        help="Cause congestion/queue buildup. ")
    parser.add_argument("-nr", "--num_runs", dest="num_runs",
                        default=NUM_EXPERIMENTS,
                        help="Number of times to cause congestion as a part of an experiment. ")
    parser.add_argument("-mz", "--multizone", dest="multi_zonal",
                        action="store_true",
                        help="Make the cluster in multiple regions. ")
    parser.add_argument("-ne", "--num-experiments", dest="num_experiments",
                        default=NUM_EXPERIMENTS,
                        help="Number of times to run an experiment. ")
    parser.add_argument("-o", "--output_file", dest="output_file",
                        default=OUTPUT_FILE,
                        help="Where to store the results of the experiments. ")
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
