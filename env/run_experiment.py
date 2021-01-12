#!/usr/bin/env python3
import argparse
import logging
import sys
import time
import os
import signal
import csv
from pathlib import Path
from multiprocessing import Process
import requests

from prometheus_api_client import PrometheusConnect
import kube_env
import util


log = logging.getLogger(__name__)

FILE_DIR = Path(__file__).parent.resolve()
RESULT_DIR = FILE_DIR.joinpath("results")
CONGESTION_PERIOD = 1607016396875512000
OUTPUT_FILE = "output.csv"
NUM_EXPERIMENTS = 1


def generate_testfolder(results_dir):
    n_folders = 0
    if os.path.isdir(results_dir):
        f_list = os.listdir(results_dir)
        n_folders = len(f_list)
    testfolder = results_dir.joinpath(f"run_{n_folders}")
    util.check_dir(testfolder)
    return testfolder


def find_congestion(platform, start_time, congestion_ts):
    if kube_env.check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    logs = query_storage(platform)
    # we want to make sure we aren't recording anything earlier
    # than our starting time. That wouldn't make sense
    ival_start = -1
    ival_end = -1
    for idx, (time_stamp, _) in enumerate(logs):
        if (int(time_stamp) - start_time) >= congestion_ts:
            ival_start = idx
            break
    for idx, (time_stamp, _) in enumerate(logs):
        if (int(time_stamp) - start_time) >= (congestion_ts + CONGESTION_PERIOD):
            ival_end = idx
            break
    log_slice = logs[ival_start:ival_end]
    congestion_dict = {}
    for idx, (time_stamp, service_name) in enumerate(log_slice):
        congestion_dict[service_name] = int(time_stamp) - start_time
        # we have congestion at more than 1 service
        if len(congestion_dict) > 1:
            for congested_service, service_ts in congestion_dict.items():
                congestion_ts_str = util.ns_to_timestamp(service_ts)
                log_str = (f"Congestion at {congested_service} "
                           f"at time {congestion_ts_str}")
                log.info(log_str)
            return min(congestion_dict.values())

    log.info("No congestion found")
    return None


def query_storage(platform):
    if platform == "GCP":
        time.sleep(10)  # wait for logs to come in
        logs = []
        cmd = f"{kube_env.TOOLS_DIR}/logs_script.sh"
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
    return sorted(logs, key=lambda tup: tup[0])


def launch_prometheus():
    if kube_env.check_kubernetes_status() != util.EXIT_SUCCESS:
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
    if kube_env.check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)
    cmd = "kubectl get pods -lapp=storage-upstream "
    cmd += " -o jsonpath={.items[0].metadata.name} -n=storage"
    storage_pod_name = util.get_output_from_proc(cmd).decode("utf-8")
    cmd = f"kubectl -n=storage port-forward {storage_pod_name} 8090:8080"
    storage_proc = util.start_process(cmd, preexec_fn=os.setsid)
    # Let settle things in a bit
    time.sleep(2)

    return storage_proc


def query_csv_loop(results_dir, prom_api, idx, start_time):
    prom_file = results_dir.joinpath(f"prom_{idx}.csv")
    csvfile = open(prom_file, "w+")
    writer = csv.writer(csvfile, delimiter=',')
    writer.writerow(["Time", "Latency"])
    while True:
        query = prom_api.custom_query(
            query="(histogram_quantile(0.90, sum(irate(istio_request_duration_milliseconds_bucket{reporter=\"source\",destination_service=~\"productpage.default.svc.cluster.local\"}[1m])) by (le)) / 1000) or histogram_quantile(0.90, sum(irate(istio_request_duration_seconds_bucket{reporter=\"source\",destination_service=~\"productpage.default.svc.cluster.local\"}[1m])) by (le))")
        for q in query:
            query_ts, latency = q["value"]
            latency = float(latency) * 1000
            query_ns = (query_ts * util.TO_NANOSECONDS) - start_time
            log.info("Time: %s 90pct Latency (ms) %s",
                     util.ns_to_timestamp(query_ns), latency)
            writer.writerow([query_ns, latency])
            csvfile.flush()
        time.sleep(1)


def check_congestion(platform, test_dir, idx, timestamps):
    start_time, congestion_ts, clear_ts = timestamps
    # open the csv file for writing
    out_file = test_dir.joinpath(f"stats_{idx}.csv")
    csv_header = ["is_congested", "congestion_start",
                  "congestion_detected", "congestion_cleared",
                  "delta_ns", "delta_s", "lat_avg"]
    csvfile = open(out_file, "w+")
    writer = csv.writer(csvfile, delimiter=',')
    writer.writerow(csv_header)

    # try to find congestion in storage
    detection_ts = find_congestion(platform, start_time, congestion_ts)
    if detection_ts is None:
        writer.writerow(["no", "." * 5])
        return
    log.info("Injected latency at %s recorded queues at %s",
             util.ns_to_timestamp(congestion_ts),
             util.ns_to_timestamp(detection_ts))
    latency = (int(detection_ts) - int(congestion_ts))
    log.info("Latency between sending and recording in storage is %s seconds",
             (latency / util.TO_NANOSECONDS))
    writer.writerow(["yes", congestion_ts, detection_ts, clear_ts, latency,
                     (latency / util.TO_NANOSECONDS), 0])
    csvfile.close()


def do_multiple_runs(platform, num_experiments, test_dir):
    # start Prometheus and wait a little to stabilize
    log.info("Forwarding Prometheus port")
    prom_proc, prom_api = launch_prometheus()
    for idx in range(int(num_experiments)):
        start_time = time.time() * util.TO_NANOSECONDS
        log.info("Starting prometheus loop")
        query_proc = Process(target=query_csv_loop,
                             args=(test_dir, prom_api, idx, start_time,))
        query_proc.start()
        log.info("Waiting a little for things to settle in...")
        time.sleep(20)
        congestion_ts = time.time() * util.TO_NANOSECONDS - start_time
        log.info("Injecting latency at time %s",
                 util.ns_to_timestamp(congestion_ts))
        kube_env.inject_failure()
        time.sleep(20)
        clear_ts = time.time() * util.TO_NANOSECONDS - start_time
        log.info("Removing latency at time %s", util.ns_to_timestamp(clear_ts))
        kube_env.remove_failure()
        time.sleep(20)
        log.info("Done at time %s", util.nano_ts(start_time))
        # process results
        timestamps = (start_time, congestion_ts, clear_ts)
        check_congestion(platform, test_dir, idx, timestamps)
        # kill prometheus
        log.info("Terminating prometheus loop")
        query_proc.terminate()
    os.killpg(os.getpgid(prom_proc.pid), signal.SIGINT)


def do_experiment(platform, num_experiments, results_dir):
    if kube_env.check_kubernetes_status() != util.EXIT_SUCCESS:
        log.error("Kubernetes is not set up."
                  " Did you run the deployment script?")
        sys.exit(util.EXIT_FAILURE)

    # generate a folder for this test
    test_dir = generate_testfolder(results_dir)

    # clean up any proc listening on 8090 and 9090 just to be safe
    util.kill_tcp_proc(9090)
    util.kill_tcp_proc(8090)

    # once everything has started, retrieve the necessary url info
    _, _, gateway_url = kube_env.get_gateway_info(platform)
    # set up storage to query later
    log.info("Forwarding storage port")
    storage_proc = launch_storage_mon()
    # start fortio load generation
    log.info("Running Fortio")
    fortio_proc = kube_env.start_fortio(gateway_url)
    time.sleep(10)
    do_multiple_runs(platform, num_experiments, test_dir)
    log.info("Killing fortio")
    # terminate fortio by sending an interrupt to the process group
    os.killpg(os.getpgid(fortio_proc.pid), signal.SIGINT)
    # kill the storage proc after the query
    log.info("Killing storage")
    os.killpg(os.getpgid(storage_proc.pid), signal.SIGINT)


def main(args):
    if args.full_run:
        result = kube_env.setup_bookinfo_deployment(
            args.platform, args.multizonal)
        if result != util.EXIT_SUCCESS:
            return result
        result = kube_env.deploy_filter(args.filter_dir)
        if result != util.EXIT_SUCCESS:
            return result
    # test the fault injection on an existing deployment
    do_experiment(args.platform, args.num_experiments, args.results_dir)
    if args.full_run:
        # all done with the test, clean up
        kube_env.stop_kubernetes(args.platform)
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
                        help="If you are running on GCP,"
                        " do you want a multi-zone cluster?")
    parser.add_argument("-f", "--full-run", dest="full_run",
                        action="store_true",
                        help="Whether to do a full run. "
                        "This includes setting up bookinfo and Kubernetes"
                        " and tearing it down again.")
    parser.add_argument("-fd", "--filter-dir", dest="filter_dir",
                        default=kube_env.FILTER_DIR,
                        help="The directory of the filter")
    parser.add_argument("-ne", "--num-experiments", dest="num_experiments",
                        default=NUM_EXPERIMENTS,
                        help="Number of times to run an experiment. ")
    parser.add_argument("-r", "--results_dir", dest="results_dir",
                        default=RESULT_DIR,
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
